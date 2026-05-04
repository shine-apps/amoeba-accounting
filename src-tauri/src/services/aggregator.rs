use crate::models::accounting_record::{AccountingRecord, AccountingResult};
use crate::models::expense_detail::ExpenseDetail;
use crate::models::labor_time::LaborTime;

/// 多维度汇总 - 将多条核算记录按周期类型汇总
///
/// 将同一周期类型的多条记录合并为汇总记录，
/// 销售额和费用累加，工时累加，人数取最大值，计算结果重新计算。
pub fn aggregate_records(records: &[AccountingRecord], period_type: &str) -> Vec<AccountingRecord> {
    if records.is_empty() {
        return vec![];
    }

    // 按周期分组（使用 period_start 作为分组键）
    let mut groups: std::collections::HashMap<String, Vec<&AccountingRecord>> =
        std::collections::HashMap::new();

    for record in records {
        groups
            .entry(record.period_start.clone())
            .or_default()
            .push(record);
    }

    let mut aggregated: Vec<AccountingRecord> = groups
        .into_iter()
        .map(|(period_start, group)| {
            let total_external_sales: f64 = group.iter().map(|r| r.external_sales).sum();
            let total_internal_sales: f64 = group.iter().map(|r| r.internal_sales).sum();
            let total_expense: f64 = group
                .iter()
                .flat_map(|r| r.expenses.iter())
                .map(|e| e.amount)
                .sum();
            let total_normal_hours: f64 = group.iter().map(|r| r.labor.normal_hours).sum();
            let total_overtime_hours: f64 = group.iter().map(|r| r.labor.overtime_hours).sum();
            let total_public_hours: f64 = group.iter().map(|r| r.labor.public_hours).sum();
            let max_headcount: i32 = group.iter().map(|r| r.labor.headcount).max().unwrap_or(1);

            let total_sales = total_external_sales + total_internal_sales;
            let added_value = total_sales - total_expense;
            let total_hours = total_normal_hours + total_overtime_hours + total_public_hours;
            let headcount = if max_headcount > 0 { max_headcount as f64 } else { 1.0 };

            let unit_value = if total_hours > 0.0 {
                added_value / total_hours
            } else {
                0.0
            };
            let sales_per_person = total_sales / headcount;
            let value_per_person = added_value / headcount;
            let value_rate = if total_sales.abs() > f64::EPSILON {
                (added_value / total_sales) * 100.0
            } else {
                0.0
            };
            let expense_rate = if total_sales.abs() > f64::EPSILON {
                (total_expense / total_sales) * 100.0
            } else {
                0.0
            };

            // 合并所有费用明细
            let all_expenses: Vec<ExpenseDetail> = group
                .iter()
                .flat_map(|r| r.expenses.clone())
                .collect();

            // 按分类汇总费用
            let mut expense_map: std::collections::HashMap<String, f64> =
                std::collections::HashMap::new();
            for expense in &all_expenses {
                *expense_map.entry(expense.category.clone()).or_insert(0.0) += expense.amount;
            }
            let aggregated_expenses: Vec<ExpenseDetail> = expense_map
                .into_iter()
                .map(|(category, amount)| ExpenseDetail {
                    id: None,
                    record_id: None,
                    category,
                    amount,
                    description: String::new(),
                })
                .collect();

            // 取最后一条记录的 period_end
            let period_end = group
                .iter()
                .map(|r| r.period_end.as_str())
                .max()
                .unwrap_or("")
                .to_string();

            AccountingRecord {
                id: None,
                amoeba_id: group[0].amoeba_id,
                period_type: period_type.to_string(),
                period_start,
                period_end,
                external_sales: total_external_sales,
                internal_sales: total_internal_sales,
                remark: format!("汇总 {} 条记录", group.len()),
                created_at: String::new(),
                updated_at: String::new(),
                expenses: aggregated_expenses,
                labor: LaborTime {
                    id: None,
                    record_id: None,
                    normal_hours: total_normal_hours,
                    overtime_hours: total_overtime_hours,
                    public_hours: total_public_hours,
                    headcount: max_headcount,
                },
                result: Some(AccountingResult {
                    total_sales,
                    total_expense,
                    added_value,
                    total_hours,
                    unit_value,
                    sales_per_person,
                    value_per_person,
                    value_rate,
                    expense_rate,
                }),
            }
        })
        .collect();

    aggregated.sort_by(|a, b| a.period_start.cmp(&b.period_start));
    aggregated
}
