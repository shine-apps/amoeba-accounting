use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmoebaCategory {
    pub id: Option<i64>,
    pub amoeba_id: i64,
    pub category_type: String,
    pub name: String,
    pub desc: String,
    pub sort_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct AmoebaCategoryInput {
    pub category_type: String,
    pub name: String,
    pub desc: String,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryList {
    pub income: Vec<AmoebaCategory>,
    pub expense: Vec<AmoebaCategory>,
}

#[derive(Debug, Deserialize)]
pub struct SaveCategoriesInput {
    pub income: Vec<AmoebaCategoryInput>,
    pub expense: Vec<AmoebaCategoryInput>,
}

pub const DEFAULT_INCOME_CATEGORIES: &[(&str, &str, i32)] = &[
    ("对外销售", "", 1),
    ("内部交易", "", 2),
    ("服务收入", "", 3),
    ("其他收入", "", 4),
];

pub const DEFAULT_EXPENSE_CATEGORIES: &[(&str, &str, i32)] = &[
    ("原材料费", "", 1),
    ("外部加工费", "", 2),
    ("电费", "", 3),
    ("折旧费", "", 4),
    ("运输费", "", 5),
    ("维修费", "", 6),
    ("办公费", "", 7),
    ("通讯费", "", 8),
    ("差旅费", "", 9),
    ("其他费用", "", 10),
];
