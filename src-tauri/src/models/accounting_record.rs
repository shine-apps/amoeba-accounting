use serde::{Deserialize, Serialize};
use super::{expense_detail::ExpenseDetail, expense_detail::ExpenseDetailInput, labor_time::LaborTime, labor_time::LaborTimeInput};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingRecord {
    pub id: Option<i64>,
    pub amoeba_id: i64,
    pub period_type: String,  // month/week/day
    pub period_start: String,
    pub period_end: String,
    pub external_sales: f64,
    pub internal_sales: f64,
    pub remark: String,
    pub created_at: String,
    pub updated_at: String,
    // 关联数据
    pub expenses: Vec<ExpenseDetail>,
    pub labor: LaborTime,
    // 计算结果
    pub result: Option<AccountingResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingResult {
    pub total_sales: f64,
    pub total_expense: f64,
    pub added_value: f64,
    pub total_hours: f64,
    pub unit_value: f64,
    pub sales_per_person: f64,
    pub value_per_person: f64,
    pub value_rate: f64,
    pub expense_rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct RecordInput {
    pub amoeba_id: i64,
    pub period_type: String,
    pub period_start: String,
    pub period_end: String,
    pub external_sales: f64,
    pub internal_sales: f64,
    pub remark: String,
    pub expenses: Vec<ExpenseDetailInput>,
    pub labor: LaborTimeInput,
}
