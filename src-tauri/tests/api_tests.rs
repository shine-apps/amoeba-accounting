#[cfg(test)]
mod tests {
    use amoeba_accounting::models::accounting_record::{AccountingRecord, AccountingResult, RecordInput};
    use amoeba_accounting::models::amoeba::AmoebaInput;
    use amoeba_accounting::models::expense_detail::{ExpenseDetail, ExpenseDetailInput};
    use amoeba_accounting::models::labor_time::{LaborTime, LaborTimeInput};
    use amoeba_accounting::repository::db;
    use amoeba_accounting::repository::amoeba_repo;
    use amoeba_accounting::repository::record_repo;
    use amoeba_accounting::repository::expense_repo;
    use amoeba_accounting::repository::labor_repo;
    use amoeba_accounting::services::calculator;
    use amoeba_accounting::services::validator;
    use amoeba_accounting::services::aggregator;
    use amoeba_accounting::commands::amoeba_cmd;
    use amoeba_accounting::commands::record_cmd;
    use amoeba_accounting::commands::export_cmd;
    use rusqlite::Connection;

    // ============================================================
    // 辅助函数：创建内存数据库
    // ============================================================
    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        db::run_migrations(&conn).unwrap();
        conn
    }

    // ============================================================
    // 核算计算引擎测试
    // ============================================================
    mod calculator_tests {
        use super::*;

        /// 测试标准场景：正常数据计算
        #[test]
        fn test_standard_calculation() {
            let expenses = vec![
                ExpenseDetailInput { category: "material".into(), amount: 400000.0, description: "原材料".into() },
                ExpenseDetailInput { category: "electricity".into(), amount: 50000.0, description: "电费".into() },
                ExpenseDetailInput { category: "depreciation".into(), amount: 30000.0, description: "折旧".into() },
            ];
            let labor = LaborTimeInput {
                normal_hours: 800.0,
                overtime_hours: 100.0,
                public_hours: 100.0,
                headcount: 10,
            };

            let result = calculator::calculate(&expenses, &labor, 800000.0, 200000.0);

            // 总销售额 = 800000 + 200000 = 1000000
            assert!((result.total_sales - 1_000_000.0).abs() < 0.01);
            // 总费用 = 400000 + 50000 + 30000 = 480000
            assert!((result.total_expense - 480_000.0).abs() < 0.01);
            // 附加价值 = 1000000 - 480000 = 520000
            assert!((result.added_value - 520_000.0).abs() < 0.01);
            // 总工时 = 800 + 100 + 100 = 1000
            assert!((result.total_hours - 1_000.0).abs() < 0.01);
            // 单位时间附加值 = 520000 / 1000 = 520
            assert!((result.unit_value - 520.0).abs() < 0.01);
            // 人均销售额 = 1000000 / 10 = 100000
            assert!((result.sales_per_person - 100_000.0).abs() < 0.01);
            // 人均附加值 = 520000 / 10 = 52000
            assert!((result.value_per_person - 52_000.0).abs() < 0.01);
            // 附加值率 = 520000 / 1000000 * 100 = 52%
            assert!((result.value_rate - 52.0).abs() < 0.01);
            // 费用率 = 480000 / 1000000 * 100 = 48%
            assert!((result.expense_rate - 48.0).abs() < 0.01);
        }

        /// 测试零销售额场景
        #[test]
        fn test_zero_sales() {
            let expenses = vec![
                ExpenseDetailInput { category: "material".into(), amount: 1000.0, description: "".into() },
            ];
            let labor = LaborTimeInput {
                normal_hours: 100.0,
                overtime_hours: 0.0,
                public_hours: 0.0,
                headcount: 5,
            };

            let result = calculator::calculate(&expenses, &labor, 0.0, 0.0);

            assert!((result.total_sales - 0.0).abs() < 0.01);
            assert!((result.total_expense - 1_000.0).abs() < 0.01);
            // 附加价值为负
            assert!((result.added_value - (-1_000.0)).abs() < 0.01);
            // 单位时间附加值 = -1000 / 100 = -10
            assert!((result.unit_value - (-10.0)).abs() < 0.01);
            // 附加值率和费用率应为 0（避免除以零）
            assert!((result.value_rate - 0.0).abs() < 0.01);
            assert!((result.expense_rate - 0.0).abs() < 0.01);
        }

        /// 测试零工时场景
        #[test]
        fn test_zero_hours() {
            let expenses = vec![];
            let labor = LaborTimeInput {
                normal_hours: 0.0,
                overtime_hours: 0.0,
                public_hours: 0.0,
                headcount: 1,
            };

            let result = calculator::calculate(&expenses, &labor, 10000.0, 0.0);

            assert!((result.total_hours - 0.0).abs() < 0.01);
            // 单位时间附加值应为 0（避免除以零）
            assert!((result.unit_value - 0.0).abs() < 0.01);
        }

        /// 测试无费用场景
        #[test]
        fn test_no_expenses() {
            let expenses: Vec<ExpenseDetailInput> = vec![];
            let labor = LaborTimeInput {
                normal_hours: 160.0,
                overtime_hours: 0.0,
                public_hours: 0.0,
                headcount: 2,
            };

            let result = calculator::calculate(&expenses, &labor, 50000.0, 0.0);

            assert!((result.total_expense - 0.0).abs() < 0.01);
            assert!((result.added_value - 50_000.0).abs() < 0.01);
            assert!((result.unit_value - 312.5).abs() < 0.01); // 50000/160
        }

        /// 测试附加价值为负（亏损场景）
        #[test]
        fn test_negative_added_value() {
            let expenses = vec![
                ExpenseDetailInput { category: "material".into(), amount: 900000.0, description: "".into() },
            ];
            let labor = LaborTimeInput {
                normal_hours: 500.0,
                overtime_hours: 0.0,
                public_hours: 0.0,
                headcount: 5,
            };

            let result = calculator::calculate(&expenses, &labor, 500000.0, 0.0);

            // 附加价值 = 500000 - 900000 = -400000
            assert!(result.added_value < 0.0);
            assert!((result.added_value - (-400_000.0)).abs() < 0.01);
            // 单位时间附加值 = -400000 / 500 = -800
            assert!((result.unit_value - (-800.0)).abs() < 0.01);
        }
    }

    // ============================================================
    // 数据校验器测试
    // ============================================================
    mod validator_tests {
        use super::*;

        fn valid_input() -> RecordInput {
            RecordInput {
                amoeba_id: 1,
                period_type: "month".into(),
                period_start: "2026-05-01".into(),
                period_end: "2026-05-31".into(),
                external_sales: 100000.0,
                internal_sales: 50000.0,
                remark: "".into(),
                expenses: vec![
                    ExpenseDetailInput { category: "material".into(), amount: 30000.0, description: "".into() },
                ],
                labor: LaborTimeInput {
                    normal_hours: 160.0,
                    overtime_hours: 20.0,
                    public_hours: 20.0,
                    headcount: 5,
                },
            }
        }

        #[test]
        fn test_valid_input_passes() {
            let input = valid_input();
            assert!(validator::validate_record(&input).is_ok());
        }

        #[test]
        fn test_invalid_amoeba_id() {
            let mut input = valid_input();
            input.amoeba_id = 0;
            assert!(validator::validate_record(&input).is_err());
            input.amoeba_id = -1;
            assert!(validator::validate_record(&input).is_err());
        }

        #[test]
        fn test_invalid_period_type() {
            let mut input = valid_input();
            input.period_type = "yearly".into();
            assert!(validator::validate_record(&input).is_err());
        }

        #[test]
        fn test_valid_period_types() {
            for pt in ["month", "week", "day"] {
                let mut input = valid_input();
                input.period_type = pt.into();
                assert!(validator::validate_record(&input).is_ok(), "period_type={} should pass", pt);
            }
        }

        #[test]
        fn test_invalid_date_format() {
            let mut input = valid_input();
            input.period_start = "2026/05/01".into();
            assert!(validator::validate_record(&input).is_err());

            input.period_start = "2026-5-1".into();
            assert!(validator::validate_record(&input).is_err());

            input.period_start = "abc".into();
            assert!(validator::validate_record(&input).is_err());
        }

        #[test]
        fn test_date_range_invalid() {
            let mut input = valid_input();
            input.period_start = "2026-06-01".into();
            input.period_end = "2026-05-31".into();
            assert!(validator::validate_record(&input).is_err());
        }

        #[test]
        fn test_negative_sales() {
            let mut input = valid_input();
            input.external_sales = -100.0;
            assert!(validator::validate_record(&input).is_err());

            input.external_sales = 100.0;
            input.internal_sales = -50.0;
            assert!(validator::validate_record(&input).is_err());
        }

        #[test]
        fn test_negative_expense() {
            let mut input = valid_input();
            input.expenses[0].amount = -100.0;
            assert!(validator::validate_record(&input).is_err());
        }

        #[test]
        fn test_empty_expense_category() {
            let mut input = valid_input();
            input.expenses[0].category = "".into();
            assert!(validator::validate_record(&input).is_err());
        }

        #[test]
        fn test_negative_hours() {
            let mut input = valid_input();
            input.labor.normal_hours = -10.0;
            assert!(validator::validate_record(&input).is_err());

            input.labor.normal_hours = 100.0;
            input.labor.overtime_hours = -5.0;
            assert!(validator::validate_record(&input).is_err());

            input.labor.overtime_hours = 0.0;
            input.labor.public_hours = -3.0;
            assert!(validator::validate_record(&input).is_err());
        }

        #[test]
        fn test_invalid_headcount() {
            let mut input = valid_input();
            input.labor.headcount = 0;
            assert!(validator::validate_record(&input).is_err());

            input.labor.headcount = -1;
            assert!(validator::validate_record(&input).is_err());
        }
    }

    // ============================================================
    // 数据库 Repository 测试
    // ============================================================
    mod repository_tests {
        use super::*;

        // --- 阿米巴 Repository ---

        #[test]
        fn test_amoeba_crud() {
            let conn = setup_db();

            // 列表为空
            let list = amoeba_repo::list(&conn).unwrap();
            assert!(list.is_empty());

            // 创建
            let input = AmoebaInput {
                name: "生产一组".into(),
                amoeba_type: "生产型".into(),
                leader: "张三".into(),
                parent_id: None,
            };
            let created = amoeba_repo::insert(&conn, &input).unwrap();
            assert!(created.id.is_some());
            assert_eq!(created.name, "生产一组");
            assert_eq!(created.amoeba_type, "生产型");
            assert_eq!(created.leader, "张三");
            assert_eq!(created.status, "active");

            // 查询
            let found = amoeba_repo::get_by_id(&conn, created.id.unwrap()).unwrap();
            assert!(found.is_some());
            assert_eq!(found.unwrap().name, "生产一组");

            // 查询不存在的
            let not_found = amoeba_repo::get_by_id(&conn, 9999).unwrap();
            assert!(not_found.is_none());

            // 更新
            let update_input = AmoebaInput {
                name: "生产一组（升级）".into(),
                amoeba_type: "生产型".into(),
                leader: "李四".into(),
                parent_id: None,
            };
            let updated = amoeba_repo::update(&conn, created.id.unwrap(), &update_input).unwrap();
            assert_eq!(updated.name, "生产一组（升级）");
            assert_eq!(updated.leader, "李四");

            // 列表
            let list = amoeba_repo::list(&conn).unwrap();
            assert_eq!(list.len(), 1);

            // 删除
            amoeba_repo::delete(&conn, created.id.unwrap()).unwrap();
            let list = amoeba_repo::list(&conn).unwrap();
            assert!(list.is_empty());
        }

        #[test]
        fn test_amoeba_parent_relationship() {
            let conn = setup_db();

            // 创建父阿米巴
            let parent = amoeba_repo::insert(&conn, &AmoebaInput {
                name: "制造部".into(),
                amoeba_type: "管理型".into(),
                leader: "王总".into(),
                parent_id: None,
            }).unwrap();

            // 创建子阿米巴
            let child = amoeba_repo::insert(&conn, &AmoebaInput {
                name: "生产一组".into(),
                amoeba_type: "生产型".into(),
                leader: "张三".into(),
                parent_id: Some(parent.id.unwrap()),
            }).unwrap();

            assert_eq!(child.parent_id, parent.id);

            // 删除父级后子级 parent_id 应为 NULL
            amoeba_repo::delete(&conn, parent.id.unwrap()).unwrap();
            let child_found = amoeba_repo::get_by_id(&conn, child.id.unwrap()).unwrap().unwrap();
            assert!(child_found.parent_id.is_none());
        }

        // --- 核算记录 Repository ---

        #[test]
        fn test_record_crud() {
            let conn = setup_db();

            // 先创建阿米巴
            let amoeba = amoeba_repo::insert(&conn, &AmoebaInput {
                name: "测试阿米巴".into(),
                amoeba_type: "生产型".into(),
                leader: "测试".into(),
                parent_id: None,
            }).unwrap();

            let result = AccountingResult {
                total_sales: 1_000_000.0,
                total_expense: 480_000.0,
                added_value: 520_000.0,
                total_hours: 1_000.0,
                unit_value: 520.0,
                sales_per_person: 100_000.0,
                value_per_person: 52_000.0,
                value_rate: 52.0,
                expense_rate: 48.0,
            };

            let record = AccountingRecord {
                id: None,
                amoeba_id: amoeba.id.unwrap(),
                period_type: "month".into(),
                period_start: "2026-05-01".into(),
                period_end: "2026-05-31".into(),
                external_sales: 800_000.0,
                internal_sales: 200_000.0,
                remark: "5月核算".into(),
                created_at: String::new(),
                updated_at: String::new(),
                expenses: vec![],
                labor: LaborTime {
                    id: None,
                    record_id: None,
                    normal_hours: 800.0,
                    overtime_hours: 100.0,
                    public_hours: 100.0,
                    headcount: 10,
                },
                result: Some(result.clone()),
            };

            // 插入
            let record_id = record_repo::insert(&conn, &record, &result).unwrap();
            assert!(record_id > 0);

            // 插入费用明细
            expense_repo::insert_batch(&conn, record_id, &[
                ExpenseDetailInput { category: "material".into(), amount: 400_000.0, description: "原材料".into() },
                ExpenseDetailInput { category: "electricity".into(), amount: 50_000.0, description: "电费".into() },
            ]).unwrap();

            // 插入工时
            labor_repo::insert(&conn, record_id, &LaborTimeInput {
                normal_hours: 800.0,
                overtime_hours: 100.0,
                public_hours: 100.0,
                headcount: 10,
            }).unwrap();

            // 查询含详情
            let found = record_repo::get_with_details(&conn, record_id).unwrap();
            assert!(found.is_some());
            let found = found.unwrap();
            assert_eq!(found.expenses.len(), 2);
            assert!((found.labor.normal_hours - 800.0).abs() < 0.01);

            // 按阿米巴查询
            let records = record_repo::list_by_amoeba(&conn, amoeba.id.unwrap()).unwrap();
            assert_eq!(records.len(), 1);

            // 按周期查询
            let queried = record_repo::query_by_period(
                &conn, amoeba.id.unwrap(), "month", "2026-05-01", "2026-05-31"
            ).unwrap();
            assert_eq!(queried.len(), 1);

            // 更新
            let mut updated_record = found.clone();
            updated_record.id = Some(record_id);
            let new_result = AccountingResult {
                total_sales: 1_100_000.0,
                total_expense: 480_000.0,
                added_value: 620_000.0,
                total_hours: 1_000.0,
                unit_value: 620.0,
                sales_per_person: 110_000.0,
                value_per_person: 62_000.0,
                value_rate: 56.36,
                expense_rate: 43.64,
            };
            record_repo::update(&conn, &updated_record, &new_result).unwrap();

            // 验证更新
            let after_update = record_repo::get_with_details(&conn, record_id).unwrap().unwrap();
            assert!((after_update.result.unwrap().total_sales - 1_100_000.0).abs() < 0.01);

            // 删除
            record_repo::delete(&conn, record_id).unwrap();
            let after_delete = record_repo::get_with_details(&conn, record_id).unwrap();
            assert!(after_delete.is_none());
        }

        #[test]
        fn test_cascade_delete() {
            let conn = setup_db();

            // 创建阿米巴 + 记录 + 费用 + 工时
            let amoeba = amoeba_repo::insert(&conn, &AmoebaInput {
                name: "级联测试".into(),
                amoeba_type: "生产型".into(),
                leader: "测试".into(),
                parent_id: None,
            }).unwrap();

            let result = AccountingResult {
                total_sales: 100.0, total_expense: 50.0, added_value: 50.0,
                total_hours: 10.0, unit_value: 5.0, sales_per_person: 10.0,
                value_per_person: 5.0, value_rate: 50.0, expense_rate: 50.0,
            };
            let record = AccountingRecord {
                id: None, amoeba_id: amoeba.id.unwrap(),
                period_type: "month".into(), period_start: "2026-01-01".into(),
                period_end: "2026-01-31".into(),
                external_sales: 100.0, internal_sales: 0.0,
                remark: "".into(), created_at: String::new(), updated_at: String::new(),
                expenses: vec![], labor: LaborTime {
                    id: None, record_id: None, normal_hours: 10.0,
                    overtime_hours: 0.0, public_hours: 0.0, headcount: 1,
                }, result: Some(result),
            };
            let record_id = record_repo::insert(&conn, &record, &record.result.as_ref().unwrap()).unwrap();
            expense_repo::insert_batch(&conn, record_id, &[
                ExpenseDetailInput { category: "material".into(), amount: 50.0, description: "".into() },
            ]).unwrap();
            labor_repo::insert(&conn, record_id, &LaborTimeInput {
                normal_hours: 10.0, overtime_hours: 0.0, public_hours: 0.0, headcount: 1,
            }).unwrap();

            // 删除阿米巴，级联删除记录
            amoeba_repo::delete(&conn, amoeba.id.unwrap()).unwrap();

            // 记录应被级联删除
            let records = record_repo::list_by_amoeba(&conn, amoeba.id.unwrap()).unwrap();
            assert!(records.is_empty());
        }
    }

    // ============================================================
    // 多维度汇总器测试
    // ============================================================
    mod aggregator_tests {
        use super::*;

        fn make_record(amoeba_id: i64, period_start: &str, period_end: &str,
                       ext_sales: f64, int_sales: f64,
                       expenses: Vec<ExpenseDetail>, labor: LaborTime) -> AccountingRecord {
            AccountingRecord {
                id: None,
                amoeba_id,
                period_type: "day".into(),
                period_start: period_start.into(),
                period_end: period_end.into(),
                external_sales: ext_sales,
                internal_sales: int_sales,
                remark: String::new(),
                created_at: String::new(),
                updated_at: String::new(),
                expenses,
                labor,
                result: None,
            }
        }

        #[test]
        fn test_aggregate_empty() {
            let result = aggregator::aggregate_records(&[], "week");
            assert!(result.is_empty());
        }

        #[test]
        fn test_aggregate_by_week() {
            let records = vec![
                make_record(1, "2026-05-05", "2026-05-05", 10000.0, 2000.0,
                    vec![ExpenseDetail { id: None, record_id: None, category: "material".into(), amount: 3000.0, description: "".into() }],
                    LaborTime { id: None, record_id: None, normal_hours: 8.0, overtime_hours: 0.0, public_hours: 0.0, headcount: 2 }),
                make_record(1, "2026-05-06", "2026-05-06", 12000.0, 3000.0,
                    vec![ExpenseDetail { id: None, record_id: None, category: "material".into(), amount: 4000.0, description: "".into() }],
                    LaborTime { id: None, record_id: None, normal_hours: 8.0, overtime_hours: 2.0, public_hours: 0.0, headcount: 2 }),
                make_record(1, "2026-05-07", "2026-05-07", 8000.0, 1000.0,
                    vec![ExpenseDetail { id: None, record_id: None, category: "material".into(), amount: 2000.0, description: "".into() }],
                    LaborTime { id: None, record_id: None, normal_hours: 8.0, overtime_hours: 0.0, public_hours: 0.0, headcount: 2 }),
            ];

            // 按日聚合应返回3条（每条一个 period_start）
            let daily = aggregator::aggregate_records(&records, "day");
            assert_eq!(daily.len(), 3);

            // 按周聚合（所有 period_start 不同，所以仍为3组）
            let weekly = aggregator::aggregate_records(&records, "week");
            assert_eq!(weekly.len(), 3);
        }

        #[test]
        fn test_aggregate_same_period() {
            let records = vec![
                make_record(1, "2026-05-01", "2026-05-01", 10000.0, 0.0,
                    vec![ExpenseDetail { id: None, record_id: None, category: "material".into(), amount: 3000.0, description: "".into() }],
                    LaborTime { id: None, record_id: None, normal_hours: 8.0, overtime_hours: 0.0, public_hours: 0.0, headcount: 2 }),
                make_record(1, "2026-05-01", "2026-05-01", 15000.0, 5000.0,
                    vec![ExpenseDetail { id: None, record_id: None, category: "material".into(), amount: 5000.0, description: "".into() }],
                    LaborTime { id: None, record_id: None, normal_hours: 8.0, overtime_hours: 2.0, public_hours: 0.0, headcount: 3 }),
            ];

            // 同一 period_start 的记录应合并为1条
            let aggregated = aggregator::aggregate_records(&records, "day");
            assert_eq!(aggregated.len(), 1);

            let agg = &aggregated[0];
            // 销售额合并：10000+15000=25000, 0+5000=5000
            assert!((agg.external_sales - 25_000.0).abs() < 0.01);
            assert!((agg.internal_sales - 5_000.0).abs() < 0.01);
            // 费用合并：3000+5000=8000
            assert!((agg.expenses[0].amount - 8_000.0).abs() < 0.01);
            // 工时合并：8+8=16, 0+2=2
            assert!((agg.labor.normal_hours - 16.0).abs() < 0.01);
            assert!((agg.labor.overtime_hours - 2.0).abs() < 0.01);
            // 人数取最大值：max(2,3)=3
            assert_eq!(agg.labor.headcount, 3);
        }
    }

    // ============================================================
    // 阿米巴命令层测试
    // ============================================================
    mod amoeba_cmd_tests {
        use super::*;

        #[test]
        fn test_create_amoeba_success() {
            let conn = setup_db();
            let input = AmoebaInput {
                name: "生产一组".into(),
                amoeba_type: "生产型".into(),
                leader: "张三".into(),
                parent_id: None,
            };
            let created = amoeba_cmd::create_amoeba_inner(&conn, &input).unwrap();
            assert!(created.id.is_some());
            assert_eq!(created.name, "生产一组");
            assert_eq!(created.amoeba_type, "生产型");
            assert_eq!(created.leader, "张三");
            assert_eq!(created.status, "active");
            assert!(!created.created_at.is_empty());
        }

        #[test]
        fn test_create_amoeba_empty_name() {
            let conn = setup_db();
            let input = AmoebaInput {
                name: "".into(),
                amoeba_type: "生产型".into(),
                leader: "张三".into(),
                parent_id: None,
            };
            let err = amoeba_cmd::create_amoeba_inner(&conn, &input).unwrap_err();
            assert!(err.contains("名称不能为空"));
        }

        #[test]
        fn test_create_amoeba_whitespace_name() {
            let conn = setup_db();
            let input = AmoebaInput {
                name: "   ".into(),
                amoeba_type: "生产型".into(),
                leader: "张三".into(),
                parent_id: None,
            };
            let err = amoeba_cmd::create_amoeba_inner(&conn, &input).unwrap_err();
            assert!(err.contains("名称不能为空"));
        }

        #[test]
        fn test_create_amoeba_invalid_type() {
            let conn = setup_db();
            let input = AmoebaInput {
                name: "测试".into(),
                amoeba_type: "无效类型".into(),
                leader: "张三".into(),
                parent_id: None,
            };
            let err = amoeba_cmd::create_amoeba_inner(&conn, &input).unwrap_err();
            assert!(err.contains("组织类型必须是以下之一"));
        }

        #[test]
        fn test_create_amoeba_all_valid_types() {
            let conn = setup_db();
            for amoeba_type in ["生产型", "营销型", "研发型", "管理型"] {
                let input = AmoebaInput {
                    name: format!("测试-{}", amoeba_type),
                    amoeba_type: amoeba_type.into(),
                    leader: "测试".into(),
                    parent_id: None,
                };
                let result = amoeba_cmd::create_amoeba_inner(&conn, &input);
                assert!(result.is_ok(), "type {} should be valid", amoeba_type);
            }
        }

        #[test]
        fn test_update_amoeba_success() {
            let conn = setup_db();
            let input = AmoebaInput {
                name: "生产一组".into(),
                amoeba_type: "生产型".into(),
                leader: "张三".into(),
                parent_id: None,
            };
            let created = amoeba_cmd::create_amoeba_inner(&conn, &input).unwrap();
            let id = created.id.unwrap();

            let update = AmoebaInput {
                name: "生产一组（升级）".into(),
                amoeba_type: "营销型".into(),
                leader: "李四".into(),
                parent_id: None,
            };
            let updated = amoeba_cmd::update_amoeba_inner(&conn, id, &update).unwrap();
            assert_eq!(updated.name, "生产一组（升级）");
            assert_eq!(updated.amoeba_type, "营销型");
            assert_eq!(updated.leader, "李四");
        }

        #[test]
        fn test_update_amoeba_empty_name() {
            let conn = setup_db();
            let input = AmoebaInput {
                name: "生产一组".into(),
                amoeba_type: "生产型".into(),
                leader: "张三".into(),
                parent_id: None,
            };
            let created = amoeba_cmd::create_amoeba_inner(&conn, &input).unwrap();

            let update = AmoebaInput {
                name: "".into(),
                amoeba_type: "生产型".into(),
                leader: "张三".into(),
                parent_id: None,
            };
            let err = amoeba_cmd::update_amoeba_inner(&conn, created.id.unwrap(), &update).unwrap_err();
            assert!(err.contains("名称不能为空"));
        }

        #[test]
        fn test_update_amoeba_invalid_type() {
            let conn = setup_db();
            let input = AmoebaInput {
                name: "生产一组".into(),
                amoeba_type: "生产型".into(),
                leader: "张三".into(),
                parent_id: None,
            };
            let created = amoeba_cmd::create_amoeba_inner(&conn, &input).unwrap();

            let update = AmoebaInput {
                name: "生产一组".into(),
                amoeba_type: "无效".into(),
                leader: "张三".into(),
                parent_id: None,
            };
            let err = amoeba_cmd::update_amoeba_inner(&conn, created.id.unwrap(), &update).unwrap_err();
            assert!(err.contains("组织类型必须是以下之一"));
        }

        #[test]
        fn test_delete_amoeba_success() {
            let conn = setup_db();
            let input = AmoebaInput {
                name: "待删除".into(),
                amoeba_type: "生产型".into(),
                leader: "测试".into(),
                parent_id: None,
            };
            let created = amoeba_cmd::create_amoeba_inner(&conn, &input).unwrap();
            let id = created.id.unwrap();

            amoeba_cmd::delete_amoeba_inner(&conn, id).unwrap();
            let list = amoeba_repo::list(&conn).unwrap();
            assert!(list.is_empty());
        }

        #[test]
        fn test_delete_nonexistent_amoeba() {
            let conn = setup_db();
            let result = amoeba_cmd::delete_amoeba_inner(&conn, 9999);
            assert!(result.is_ok());
        }

        #[test]
        fn test_list_amoebas() {
            let conn = setup_db();
            assert!(amoeba_cmd::list_amoebas_inner(&conn).unwrap().is_empty());

            let input1 = AmoebaInput {
                name: "一组".into(), amoeba_type: "生产型".into(),
                leader: "A".into(), parent_id: None,
            };
            let input2 = AmoebaInput {
                name: "二组".into(), amoeba_type: "营销型".into(),
                leader: "B".into(), parent_id: None,
            };
            amoeba_cmd::create_amoeba_inner(&conn, &input1).unwrap();
            amoeba_cmd::create_amoeba_inner(&conn, &input2).unwrap();

            let list = amoeba_cmd::list_amoebas_inner(&conn).unwrap();
            assert_eq!(list.len(), 2);
            assert_eq!(list[0].name, "一组");
            assert_eq!(list[1].name, "二组");
        }
    }

    // ============================================================
    // 核算记录命令层测试
    // ============================================================
    mod record_cmd_tests {
        use super::*;

        fn create_test_amoeba(conn: &Connection) -> i64 {
            let input = AmoebaInput {
                name: "测试阿米巴".into(),
                amoeba_type: "生产型".into(),
                leader: "测试".into(),
                parent_id: None,
            };
            amoeba_cmd::create_amoeba_inner(conn, &input).unwrap().id.unwrap()
        }

        fn valid_record_input(amoeba_id: i64) -> RecordInput {
            RecordInput {
                amoeba_id,
                period_type: "month".into(),
                period_start: "2026-05-01".into(),
                period_end: "2026-05-31".into(),
                external_sales: 800_000.0,
                internal_sales: 200_000.0,
                remark: "5月核算".into(),
                expenses: vec![
                    ExpenseDetailInput { category: "material".into(), amount: 400_000.0, description: "原材料".into() },
                    ExpenseDetailInput { category: "electricity".into(), amount: 50_000.0, description: "电费".into() },
                ],
                labor: LaborTimeInput {
                    normal_hours: 800.0,
                    overtime_hours: 100.0,
                    public_hours: 100.0,
                    headcount: 10,
                },
            }
        }

        #[test]
        fn test_save_record_create() {
            let conn = setup_db();
            let amoeba_id = create_test_amoeba(&conn);
            let input = valid_record_input(amoeba_id);

            let result = record_cmd::save_record_inner(&conn, None, &input).unwrap();

            // 验证计算结果
            assert!((result.total_sales - 1_000_000.0).abs() < 0.01);
            assert!((result.total_expense - 450_000.0).abs() < 0.01);
            assert!((result.added_value - 550_000.0).abs() < 0.01);
            assert!((result.total_hours - 1_000.0).abs() < 0.01);

            // 验证记录已持久化
            let records = record_cmd::list_records_inner(&conn, amoeba_id).unwrap();
            assert_eq!(records.len(), 1);
            assert_eq!(records[0].expenses.len(), 2);
            assert!((records[0].labor.normal_hours - 800.0).abs() < 0.01);
        }

        #[test]
        fn test_save_record_create_zero_record_id() {
            let conn = setup_db();
            let amoeba_id = create_test_amoeba(&conn);
            let input = valid_record_input(amoeba_id);

            // record_id=Some(0) 应走新建路径
            let result = record_cmd::save_record_inner(&conn, Some(0), &input).unwrap();
            assert!(result.total_sales > 0.0);
            assert_eq!(record_cmd::list_records_inner(&conn, amoeba_id).unwrap().len(), 1);
        }

        #[test]
        fn test_save_record_create_negative_record_id() {
            let conn = setup_db();
            let amoeba_id = create_test_amoeba(&conn);
            let input = valid_record_input(amoeba_id);

            // record_id=Some(-1) 应走新建路径
            let result = record_cmd::save_record_inner(&conn, Some(-1), &input).unwrap();
            assert!(result.total_sales > 0.0);
            assert_eq!(record_cmd::list_records_inner(&conn, amoeba_id).unwrap().len(), 1);
        }

        #[test]
        fn test_save_record_update() {
            let conn = setup_db();
            let amoeba_id = create_test_amoeba(&conn);
            let input = valid_record_input(amoeba_id);

            // 先创建
            record_cmd::save_record_inner(&conn, None, &input).unwrap();
            let records = record_cmd::list_records_inner(&conn, amoeba_id).unwrap();
            let record_id = records[0].id.unwrap();

            // 更新：修改销售额和费用
            let mut update_input = valid_record_input(amoeba_id);
            update_input.external_sales = 900_000.0;
            update_input.internal_sales = 100_000.0;
            update_input.expenses = vec![
                ExpenseDetailInput { category: "material".into(), amount: 300_000.0, description: "新原材料".into() },
            ];
            update_input.labor = LaborTimeInput {
                normal_hours: 700.0,
                overtime_hours: 150.0,
                public_hours: 150.0,
                headcount: 8,
            };

            let result = record_cmd::save_record_inner(&conn, Some(record_id), &update_input).unwrap();

            // 验证新结果
            assert!((result.total_sales - 1_000_000.0).abs() < 0.01);
            assert!((result.total_expense - 300_000.0).abs() < 0.01);

            // 验证持久化：费用已被替换
            let updated = record_cmd::get_record_inner(&conn, record_id).unwrap().unwrap();
            assert_eq!(updated.expenses.len(), 1);
            assert_eq!(updated.expenses[0].category, "material");
            assert_eq!(updated.expenses[0].description, "新原材料");
            assert!((updated.labor.normal_hours - 700.0).abs() < 0.01);
            assert!((updated.labor.overtime_hours - 150.0).abs() < 0.01);
        }

        #[test]
        fn test_save_record_update_nonexistent() {
            let conn = setup_db();
            let amoeba_id = create_test_amoeba(&conn);
            let input = valid_record_input(amoeba_id);

            let err = record_cmd::save_record_inner(&conn, Some(9999), &input).unwrap_err();
            assert!(err.contains("不存在"));
        }

        #[test]
        fn test_save_record_validation_fails() {
            let conn = setup_db();
            let amoeba_id = create_test_amoeba(&conn);
            let mut input = valid_record_input(amoeba_id);
            input.amoeba_id = 0; // 无效

            let err = record_cmd::save_record_inner(&conn, None, &input).unwrap_err();
            assert!(err.contains("阿米巴组织 ID"));
        }

        #[test]
        fn test_save_record_update_expenses_replaced() {
            let conn = setup_db();
            let amoeba_id = create_test_amoeba(&conn);
            let input = valid_record_input(amoeba_id);

            record_cmd::save_record_inner(&conn, None, &input).unwrap();
            let records = record_cmd::list_records_inner(&conn, amoeba_id).unwrap();
            let record_id = records[0].id.unwrap();

            // 更新为完全不同的费用明细
            let mut update_input = valid_record_input(amoeba_id);
            update_input.expenses = vec![
                ExpenseDetailInput { category: "rent".into(), amount: 100_000.0, description: "租金".into() },
                ExpenseDetailInput { category: "travel".into(), amount: 20_000.0, description: "差旅".into() },
                ExpenseDetailInput { category: "depreciation".into(), amount: 15_000.0, description: "折旧".into() },
            ];

            record_cmd::save_record_inner(&conn, Some(record_id), &update_input).unwrap();

            let updated = record_cmd::get_record_inner(&conn, record_id).unwrap().unwrap();
            assert_eq!(updated.expenses.len(), 3);
            let categories: Vec<&str> = updated.expenses.iter().map(|e| e.category.as_str()).collect();
            assert!(categories.contains(&"rent"));
            assert!(categories.contains(&"travel"));
            assert!(categories.contains(&"depreciation"));
            assert!(!categories.contains(&"material")); // 旧费用已被删除
        }

        #[test]
        fn test_list_records() {
            let conn = setup_db();
            let amoeba_id = create_test_amoeba(&conn);
            let input = valid_record_input(amoeba_id);

            record_cmd::save_record_inner(&conn, None, &input).unwrap();

            let records = record_cmd::list_records_inner(&conn, amoeba_id).unwrap();
            assert_eq!(records.len(), 1);
            assert_eq!(records[0].expenses.len(), 2);
            assert!((records[0].labor.headcount - 10) == 0);
        }

        #[test]
        fn test_list_records_empty_amoeba() {
            let conn = setup_db();
            let amoeba_id = create_test_amoeba(&conn);

            let records = record_cmd::list_records_inner(&conn, amoeba_id).unwrap();
            assert!(records.is_empty());
        }

        #[test]
        fn test_get_record_exists() {
            let conn = setup_db();
            let amoeba_id = create_test_amoeba(&conn);
            let input = valid_record_input(amoeba_id);

            record_cmd::save_record_inner(&conn, None, &input).unwrap();
            let records = record_cmd::list_records_inner(&conn, amoeba_id).unwrap();
            let id = records[0].id.unwrap();

            let found = record_cmd::get_record_inner(&conn, id).unwrap();
            assert!(found.is_some());
            let found = found.unwrap();
            assert_eq!(found.period_type, "month");
            assert!(!found.expenses.is_empty());
        }

        #[test]
        fn test_get_record_nonexistent() {
            let conn = setup_db();
            let found = record_cmd::get_record_inner(&conn, 9999).unwrap();
            assert!(found.is_none());
        }

        #[test]
        fn test_delete_record_success() {
            let conn = setup_db();
            let amoeba_id = create_test_amoeba(&conn);
            let input = valid_record_input(amoeba_id);

            record_cmd::save_record_inner(&conn, None, &input).unwrap();
            let records = record_cmd::list_records_inner(&conn, amoeba_id).unwrap();
            let id = records[0].id.unwrap();

            record_cmd::delete_record_inner(&conn, id).unwrap();
            assert!(record_cmd::get_record_inner(&conn, id).unwrap().is_none());
        }

        #[test]
        fn test_delete_nonexistent_record() {
            let conn = setup_db();
            let result = record_cmd::delete_record_inner(&conn, 9999);
            assert!(result.is_ok());
        }
    }

    // ============================================================
    // Excel 导出命令层测试
    // ============================================================
    mod export_cmd_tests {
        use super::*;
        use std::fs;

        fn temp_output_path(name: &str) -> String {
            let mut path = std::env::temp_dir();
            path.push(format!("amoeba_test_{}", name));
            path.to_str().unwrap().to_string()
        }

        fn create_amoeba_with_record(conn: &Connection) -> (i64, i64) {
            let amoeba = amoeba_cmd::create_amoeba_inner(conn, &AmoebaInput {
                name: "导出测试阿米巴".into(),
                amoeba_type: "生产型".into(),
                leader: "测试".into(),
                parent_id: None,
            }).unwrap();
            let amoeba_id = amoeba.id.unwrap();

            let input = RecordInput {
                amoeba_id,
                period_type: "month".into(),
                period_start: "2026-05-01".into(),
                period_end: "2026-05-31".into(),
                external_sales: 500_000.0,
                internal_sales: 100_000.0,
                remark: "导出测试".into(),
                expenses: vec![
                    ExpenseDetailInput { category: "material".into(), amount: 200_000.0, description: "原材料".into() },
                ],
                labor: LaborTimeInput {
                    normal_hours: 160.0,
                    overtime_hours: 20.0,
                    public_hours: 0.0,
                    headcount: 5,
                },
            };
            record_cmd::save_record_inner(conn, None, &input).unwrap();
            let records = record_cmd::list_records_inner(conn, amoeba_id).unwrap();
            let record_id = records[0].id.unwrap();
            (amoeba_id, record_id)
        }

        #[test]
        fn test_export_amoeba_not_found() {
            let conn = setup_db();
            let output = temp_output_path("notfound");
            let result = export_cmd::export_excel_inner(
                &conn, 9999, None, None, None, &output,
            );
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("不存在"));
        }

        #[test]
        fn test_export_no_records() {
            let conn = setup_db();
            let amoeba = amoeba_cmd::create_amoeba_inner(&conn, &AmoebaInput {
                name: "空阿米巴".into(),
                amoeba_type: "生产型".into(),
                leader: "测试".into(),
                parent_id: None,
            }).unwrap();
            let output = temp_output_path("empty");

            let result = export_cmd::export_excel_inner(
                &conn, amoeba.id.unwrap(), None, None, None, &output,
            );
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("没有可导出的核算记录"));
        }

        #[test]
        fn test_export_appends_xlsx_extension() {
            let conn = setup_db();
            let (amoeba_id, _) = create_amoeba_with_record(&conn);
            let output = temp_output_path("append_test");

            let result = export_cmd::export_excel_inner(
                &conn, amoeba_id, None, None, None, &output,
            ).unwrap();

            assert!(result.ends_with(".xlsx"));
            // 验证文件确实存在
            assert!(std::path::Path::new(&result).exists());
            fs::remove_file(&result).ok();
        }

        #[test]
        fn test_export_with_period_filter() {
            let conn = setup_db();
            let (amoeba_id, _) = create_amoeba_with_record(&conn);

            // 再创建一条不同月份的记录
            let input = RecordInput {
                amoeba_id,
                period_type: "month".into(),
                period_start: "2026-06-01".into(),
                period_end: "2026-06-30".into(),
                external_sales: 300_000.0,
                internal_sales: 50_000.0,
                remark: "6月核算".into(),
                expenses: vec![
                    ExpenseDetailInput { category: "material".into(), amount: 150_000.0, description: "原材料".into() },
                ],
                labor: LaborTimeInput {
                    normal_hours: 160.0,
                    overtime_hours: 10.0,
                    public_hours: 0.0,
                    headcount: 5,
                },
            };
            record_cmd::save_record_inner(&conn, None, &input).unwrap();

            let output = temp_output_path("period_filter");
            let result = export_cmd::export_excel_inner(
                &conn, amoeba_id,
                Some("month"), Some("2026-05-01"), Some("2026-05-31"),
                &output,
            ).unwrap();

            assert!(std::path::Path::new(&result).exists());
            fs::remove_file(&result).ok();
        }

        #[test]
        fn test_export_success() {
            let conn = setup_db();
            let (amoeba_id, _) = create_amoeba_with_record(&conn);
            let output = temp_output_path("success");

            let result = export_cmd::export_excel_inner(
                &conn, amoeba_id, None, None, None, &output,
            ).unwrap();

            assert!(output.ends_with(".xlsx") || result.ends_with(".xlsx"));
            let path = std::path::Path::new(&result);
            assert!(path.exists());
            // xlsx 文件不应为空
            let metadata = fs::metadata(&result).unwrap();
            assert!(metadata.len() > 0);
            fs::remove_file(&result).ok();
        }
    }

    // ============================================================
    // 校验器边界条件测试
    // ============================================================
    mod validator_edge_case_tests {
        use super::*;

        fn valid_input() -> RecordInput {
            RecordInput {
                amoeba_id: 1,
                period_type: "month".into(),
                period_start: "2026-05-01".into(),
                period_end: "2026-05-31".into(),
                external_sales: 100_000.0,
                internal_sales: 50_000.0,
                remark: "".into(),
                expenses: vec![
                    ExpenseDetailInput { category: "material".into(), amount: 30_000.0, description: "".into() },
                ],
                labor: LaborTimeInput {
                    normal_hours: 160.0,
                    overtime_hours: 20.0,
                    public_hours: 20.0,
                    headcount: 5,
                },
            }
        }

        #[test]
        fn test_empty_expenses_list_valid() {
            let mut input = valid_input();
            input.expenses = vec![];
            assert!(validator::validate_record(&input).is_ok());
        }

        #[test]
        fn test_zero_sales_valid() {
            let mut input = valid_input();
            input.external_sales = 0.0;
            input.internal_sales = 0.0;
            assert!(validator::validate_record(&input).is_ok());
        }

        #[test]
        fn test_empty_description_valid() {
            let mut input = valid_input();
            input.expenses[0].description = "".into();
            assert!(validator::validate_record(&input).is_ok());
        }
    }

    // ============================================================
    // 仓库层边界条件测试
    // ============================================================
    mod repository_edge_case_tests {
        use super::*;

        #[test]
        fn test_record_update_preserves_created_at() {
            let conn = setup_db();

            let amoeba = amoeba_repo::insert(&conn, &AmoebaInput {
                name: "测试".into(),
                amoeba_type: "生产型".into(),
                leader: "测试".into(),
                parent_id: None,
            }).unwrap();

            let result = AccountingResult {
                total_sales: 100_000.0, total_expense: 50_000.0, added_value: 50_000.0,
                total_hours: 100.0, unit_value: 500.0, sales_per_person: 10_000.0,
                value_per_person: 5_000.0, value_rate: 50.0, expense_rate: 50.0,
            };
            let record = AccountingRecord {
                id: None,
                amoeba_id: amoeba.id.unwrap(),
                period_type: "month".into(),
                period_start: "2026-01-01".into(),
                period_end: "2026-01-31".into(),
                external_sales: 100_000.0, internal_sales: 0.0,
                remark: "".into(),
                created_at: String::new(), updated_at: String::new(),
                expenses: vec![],
                labor: LaborTime {
                    id: None, record_id: None,
                    normal_hours: 100.0, overtime_hours: 0.0,
                    public_hours: 0.0, headcount: 1,
                },
                result: Some(result.clone()),
            };

            let record_id = record_repo::insert(&conn, &record, &result).unwrap();

            // 读取创建时的时间戳
            let created = record_repo::get_with_details(&conn, record_id).unwrap().unwrap();
            let created_at = created.created_at.clone();
            assert!(!created_at.is_empty());

            // 更新
            let mut updated = created.clone();
            updated.id = Some(record_id);
            let new_result = AccountingResult {
                total_sales: 200_000.0, total_expense: 50_000.0, added_value: 150_000.0,
                total_hours: 100.0, unit_value: 1_500.0, sales_per_person: 20_000.0,
                value_per_person: 15_000.0, value_rate: 75.0, expense_rate: 25.0,
            };
            record_repo::update(&conn, &updated, &new_result).unwrap();

            let after = record_repo::get_with_details(&conn, record_id).unwrap().unwrap();
            // created_at 应保持不变
            assert_eq!(after.created_at, created_at);
            // updated_at 应被设置
            assert!(!after.updated_at.is_empty());
        }

        #[test]
        fn test_list_records_order() {
            let conn = setup_db();

            let amoeba = amoeba_repo::insert(&conn, &AmoebaInput {
                name: "排序测试".into(),
                amoeba_type: "生产型".into(),
                leader: "测试".into(),
                parent_id: None,
            }).unwrap();
            let amoeba_id = amoeba.id.unwrap();

            let result = AccountingResult {
                total_sales: 100.0, total_expense: 50.0, added_value: 50.0,
                total_hours: 10.0, unit_value: 5.0, sales_per_person: 10.0,
                value_per_person: 5.0, value_rate: 50.0, expense_rate: 50.0,
            };

            // 创建三条不同月份的数据
            for month in ["03", "04", "05"] {
                let record = AccountingRecord {
                    id: None, amoeba_id,
                    period_type: "month".into(),
                    period_start: format!("2026-{}-01", month),
                    period_end: format!("2026-{}-31", month),
                    external_sales: 100.0, internal_sales: 0.0,
                    remark: "".into(),
                    created_at: String::new(), updated_at: String::new(),
                    expenses: vec![],
                    labor: LaborTime {
                        id: None, record_id: None,
                        normal_hours: 10.0, overtime_hours: 0.0,
                        public_hours: 0.0, headcount: 1,
                    },
                    result: Some(result.clone()),
                };
                let rid = record_repo::insert(&conn, &record, &result).unwrap();
                labor_repo::insert(&conn, rid, &LaborTimeInput {
                    normal_hours: 10.0, overtime_hours: 0.0,
                    public_hours: 0.0, headcount: 1,
                }).unwrap();
            }

            let records = record_repo::list_by_amoeba(&conn, amoeba_id).unwrap();
            assert_eq!(records.len(), 3);
            // 应按 period_start DESC 排序
            assert_eq!(records[0].period_start, "2026-05-01");
            assert_eq!(records[1].period_start, "2026-04-01");
            assert_eq!(records[2].period_start, "2026-03-01");
        }

        #[test]
        fn test_labor_get_nonexistent_record() {
            let conn = setup_db();
            let labor = labor_repo::get_by_record(&conn, 9999).unwrap();
            assert!(labor.is_none());
        }

        #[test]
        fn test_expense_delete_by_record_empty() {
            let conn = setup_db();
            // 即使没有费用明细，delete 也应成功
            let result = expense_repo::delete_by_record(&conn, 9999);
            assert!(result.is_ok());
        }
    }
}
