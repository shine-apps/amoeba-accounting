use crate::models::accounting_record::RecordInput;

/// 数据校验 - 校验核算记录输入的所有字段
pub fn validate_record(input: &RecordInput) -> Result<(), String> {
    // 校验阿米巴 ID
    if input.amoeba_id <= 0 {
        return Err("阿米巴组织 ID 必须大于 0".to_string());
    }

    // 校验周期类型
    let valid_period_types = ["month", "week", "day"];
    if !valid_period_types.contains(&input.period_type.as_str()) {
        return Err(format!(
            "周期类型必须是以下之一: {}",
            valid_period_types.join(", ")
        ));
    }

    // 校验日期格式 (YYYY-MM-DD)
    if !is_valid_date(&input.period_start) {
        return Err("周期开始日期格式无效，请使用 YYYY-MM-DD 格式".to_string());
    }
    if !is_valid_date(&input.period_end) {
        return Err("周期结束日期格式无效，请使用 YYYY-MM-DD 格式".to_string());
    }

    // 校验日期范围
    if input.period_start > input.period_end {
        return Err("周期开始日期不能晚于结束日期".to_string());
    }

    // 校验收入明细
    for (i, income) in input.income_details.iter().enumerate() {
        if income.category.trim().is_empty() {
            return Err(format!("第 {} 项收入明细的分类不能为空", i + 1));
        }
        if income.amount < 0.0 {
            return Err(format!("第 {} 项收入明细的金额不能为负数", i + 1));
        }
    }

    // 校验费用明细
    for (i, expense) in input.expenses.iter().enumerate() {
        if expense.category.trim().is_empty() {
            return Err(format!("第 {} 项费用明细的分类不能为空", i + 1));
        }
        if expense.amount < 0.0 {
            return Err(format!("第 {} 项费用明细的金额不能为负数", i + 1));
        }
    }

    // 校验工时数据
    if input.labor.normal_hours < 0.0 {
        return Err("正常工时不能为负数".to_string());
    }
    if input.labor.overtime_hours < 0.0 {
        return Err("加班工时不能为负数".to_string());
    }
    if input.labor.public_hours < 0.0 {
        return Err("公共工时不能为负数".to_string());
    }
    if input.labor.headcount <= 0 {
        return Err("人数必须大于 0".to_string());
    }

    Ok(())
}

/// 校验日期格式是否为 YYYY-MM-DD
fn is_valid_date(date_str: &str) -> bool {
    if date_str.len() != 10 {
        return false;
    }
    let bytes = date_str.as_bytes();
    // 检查分隔符
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    // 检查各部分是否为数字
    for i in 0..10 {
        if i == 4 || i == 7 {
            continue;
        }
        if !bytes[i].is_ascii_digit() {
            return false;
        }
    }
    true
}
