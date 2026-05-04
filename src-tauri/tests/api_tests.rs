#[cfg(test)]
mod tests {
    use amoeba_accounting::models::accounting_record::{AccountingRecord, AccountingResult, RecordInput};
    use amoeba_accounting::models::amoeba::{Amoeba, AmoebaInput};
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
}
