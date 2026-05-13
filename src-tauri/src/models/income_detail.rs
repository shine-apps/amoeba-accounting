use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeDetail {
    pub id: Option<i64>,
    pub record_id: Option<i64>,
    pub category: String,
    pub amount: f64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct IncomeDetailInput {
    pub category: String,
    pub amount: f64,
    pub description: String,
}
