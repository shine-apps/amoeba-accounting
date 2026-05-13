use crate::models::accounting_record::AccountingResult;
use crate::models::expense_detail::ExpenseDetailInput;
use crate::models::income_detail::IncomeDetailInput;
use crate::models::labor_time::LaborTimeInput;

/// 核算计算引擎 - 实现阿米巴经营核算的 9 个核心公式
///
/// 公式说明：
/// 1. 总销售额 = SUM(各项收入明细金额)
/// 2. 总费用 = SUM(各项费用明细金额)
/// 3. 附加价值 = 总销售额 - 总费用
/// 4. 总劳动时间 = 正常工时 + 加班工时 + 公共工时
/// 5. 单位时间附加值 = 附加价值 / 总劳动时间
/// 6. 人均销售额 = 总销售额 / 人数
/// 7. 人均附加值 = 附加价值 / 人数
/// 8. 附加值率 = 附加价值 / 总销售额 * 100
/// 9. 费用率 = 总费用 / 总销售额 * 100
pub fn calculate(
    income_details: &[IncomeDetailInput],
    expenses: &[ExpenseDetailInput],
    labor: &LaborTimeInput,
) -> AccountingResult {
    // 公式1: 总销售额 = SUM(各项收入明细金额)
    let total_sales: f64 = income_details.iter().map(|i| i.amount).sum();

    // 公式2: 总费用 = SUM(各项费用明细金额)
    let total_expense: f64 = expenses.iter().map(|e| e.amount).sum();

    // 公式3: 附加价值 = 总销售额 - 总费用
    let added_value = total_sales - total_expense;

    // 公式4: 总劳动时间 = 正常工时 + 加班工时 + 公共工时
    let total_hours = labor.normal_hours + labor.overtime_hours + labor.public_hours;

    // 公式5: 单位时间附加值 = 附加价值 / 总劳动时间
    let unit_value = if total_hours > 0.0 {
        added_value / total_hours
    } else {
        0.0
    };

    // 公式6: 人均销售额 = 总销售额 / 人数
    let headcount = if labor.headcount > 0 { labor.headcount as f64 } else { 1.0 };
    let sales_per_person = total_sales / headcount;

    // 公式7: 人均附加值 = 附加价值 / 人数
    let value_per_person = added_value / headcount;

    // 公式8: 附加值率 = 附加价值 / 总销售额 * 100
    let value_rate = if total_sales.abs() > f64::EPSILON {
        (added_value / total_sales) * 100.0
    } else {
        0.0
    };

    // 公式9: 费用率 = 总费用 / 总销售额 * 100
    let expense_rate = if total_sales.abs() > f64::EPSILON {
        (total_expense / total_sales) * 100.0
    } else {
        0.0
    };

    AccountingResult {
        total_sales,
        total_expense,
        added_value,
        total_hours,
        unit_value,
        sales_per_person,
        value_per_person,
        value_rate,
        expense_rate,
    }
}
