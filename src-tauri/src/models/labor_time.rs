use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborTime {
    pub id: Option<i64>,
    pub record_id: Option<i64>,
    pub normal_hours: f64,
    pub overtime_hours: f64,
    pub public_hours: f64,
    pub headcount: i32,
}

#[derive(Debug, Deserialize)]
pub struct LaborTimeInput {
    pub normal_hours: f64,
    pub overtime_hours: f64,
    pub public_hours: f64,
    pub headcount: i32,
}
