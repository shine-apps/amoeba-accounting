use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseDetail {
    pub id: Option<i64>,
    pub record_id: Option<i64>,
    pub category: String,
    pub amount: f64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ExpenseDetailInput {
    pub category: String,
    pub amount: f64,
    pub description: String,
}
