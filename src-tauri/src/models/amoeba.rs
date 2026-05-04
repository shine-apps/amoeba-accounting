use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amoeba {
    pub id: Option<i64>,
    pub name: String,
    pub amoeba_type: String,  // 生产型/营销型/研发型/管理型
    pub leader: String,
    pub parent_id: Option<i64>,
    pub status: String,  // active/inactive
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct AmoebaInput {
    pub name: String,
    pub amoeba_type: String,
    pub leader: String,
    pub parent_id: Option<i64>,
}
