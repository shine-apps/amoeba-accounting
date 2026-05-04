use rust_xlsxwriter::{Workbook, Worksheet};
use crate::models::accounting_record::AccountingRecord;
use crate::models::amoeba::Amoeba;
use super::styles;

/// 将核算记录导出为 Excel 文件
///
/// 包含3个Sheet：
/// 1. 核算表 - 主核算数据
/// 2. 费用明细 - 各记录的费用明细
/// 3. 趋势分析 - 各期间的指标趋势
pub fn write_excel(
    records: &[AccountingRecord],
    amoeba: &Amoeba,
    output_path: &str,
) -> Result<(), String> {
    let mut workbook = Workbook::new();

    // Sheet1: 核算表
    let sheet1 = workbook
        .add_worksheet()
        .set_name("核算表")
        .map_err(|e| format!("创建工作表失败: {}", e))?;
    write_accounting_sheet(sheet1, records, amoeba)?;

    // Sheet2: 费用明细
    let sheet2 = workbook
        .add_worksheet()
        .set_name("费用明细")
        .map_err(|e| format!("创建工作表失败: {}", e))?;
    write_expense_sheet(sheet2, records, amoeba)?;

    // Sheet3: 趋势分析
    let sheet3 = workbook
        .add_worksheet()
        .set_name("趋势分析")
        .map_err(|e| format!("创建工作表失败: {}", e))?;
    write_trend_sheet(sheet3, records, amoeba)?;

    // 保存文件
    workbook
        .save(output_path)
        .map_err(|e| format!("保存 Excel 文件失败: {}", e))?;

    Ok(())
}

/// Sheet1: 核算表
fn write_accounting_sheet(
    sheet: &mut Worksheet,
    records: &[AccountingRecord],
    amoeba: &Amoeba,
) -> Result<(), String> {
    let title_fmt = styles::title_format();
    let header_fmt = styles::header_format();
    let normal_fmt = styles::normal_format();
    let money_fmt = styles::money_format();
    let neg_money_fmt = styles::negative_money_format();
    let percent_fmt = styles::percent_format();
    let total_fmt = styles::total_format();
    let total_money_fmt = styles::total_money_format();
    let neg_total_money_fmt = styles::negative_total_money_format();
    let total_pct_fmt = styles::total_percent_format();
    let highlight_money_fmt = styles::highlight_money_format();

    // 设置列宽
    sheet
        .set_column_width(0, 14)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(1, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(2, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(3, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(4, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(5, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(6, 14)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(7, 18)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(8, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(9, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(10, 14)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(11, 14)
        .map_err(|e| format!("设置列宽失败: {}", e))?;

    // 标题行
    let title = format!("{} - 阿米巴单位时间核算表", amoeba.name);
    sheet
        .merge_range(0, 0, 0, 11, title.as_str(), &title_fmt)
        .map_err(|e| format!("写入标题失败: {}", e))?;

    // 副标题行
    let subtitle = format!(
        "组织类型: {} | 负责人: {} | 导出时间: {}",
        amoeba.amoeba_type,
        amoeba.leader,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    sheet
        .merge_range(1, 0, 1, 11, subtitle.as_str(), &header_fmt)
        .map_err(|e| format!("写入副标题失败: {}", e))?;

    // 表头行
    let headers = [
        "期间",
        "外部销售额",
        "内部销售额",
        "总销售额",
        "总费用",
        "附加价值",
        "总工时(h)",
        "单位时间附加值",
        "人均销售额",
        "人均附加值",
        "附加值率(%)",
        "费用率(%)",
    ];
    for (col, header) in headers.iter().enumerate() {
        sheet
            .write_string_with_format(2, col as u16, *header, &header_fmt)
            .map_err(|e| format!("写入表头失败: {}", e))?;
    }

    // 数据行
    let mut row = 3u32;
    let mut sum_external_sales = 0.0_f64;
    let mut sum_internal_sales = 0.0_f64;
    let mut sum_total_sales = 0.0_f64;
    let mut sum_total_expense = 0.0_f64;
    let mut sum_added_value = 0.0_f64;
    let mut sum_total_hours = 0.0_f64;

    for record in records {
        let result = match &record.result {
            Some(r) => r,
            None => continue,
        };

        sheet
            .write_string_with_format(row, 0, &record.period_start, &normal_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 1, record.external_sales, &money_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 2, record.internal_sales, &money_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 3, result.total_sales, &money_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;

        let exp_fmt = if result.total_expense < 0.0 { &neg_money_fmt } else { &money_fmt };
        sheet
            .write_number_with_format(row, 4, result.total_expense, exp_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;

        let av_fmt = if result.added_value < 0.0 { &neg_money_fmt } else { &money_fmt };
        sheet
            .write_number_with_format(row, 5, result.added_value, av_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 6, result.total_hours, &normal_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;

        let uv_fmt = if result.unit_value < 0.0 {
            &neg_money_fmt
        } else {
            &highlight_money_fmt
        };
        sheet
            .write_number_with_format(row, 7, result.unit_value, uv_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 8, result.sales_per_person, &money_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;

        let vp_fmt = if result.value_per_person < 0.0 {
            &neg_money_fmt
        } else {
            &money_fmt
        };
        sheet
            .write_number_with_format(row, 9, result.value_per_person, vp_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 10, result.value_rate, &percent_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 11, result.expense_rate, &percent_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;

        sum_external_sales += record.external_sales;
        sum_internal_sales += record.internal_sales;
        sum_total_sales += result.total_sales;
        sum_total_expense += result.total_expense;
        sum_added_value += result.added_value;
        sum_total_hours += result.total_hours;

        row += 1;
    }

    // 合计行
    if !records.is_empty() {
        sheet
            .write_string_with_format(row, 0, "合计", &total_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;
        sheet
            .write_number_with_format(row, 1, sum_external_sales, &total_money_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;
        sheet
            .write_number_with_format(row, 2, sum_internal_sales, &total_money_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;
        sheet
            .write_number_with_format(row, 3, sum_total_sales, &total_money_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;

        let te_fmt = if sum_total_expense < 0.0 {
            &neg_total_money_fmt
        } else {
            &total_money_fmt
        };
        sheet
            .write_number_with_format(row, 4, sum_total_expense, te_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;

        let av_fmt = if sum_added_value < 0.0 {
            &neg_total_money_fmt
        } else {
            &total_money_fmt
        };
        sheet
            .write_number_with_format(row, 5, sum_added_value, av_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;
        sheet
            .write_number_with_format(row, 6, sum_total_hours, &total_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;

        let avg_unit = if sum_total_hours > 0.0 {
            sum_added_value / sum_total_hours
        } else {
            0.0
        };
        sheet
            .write_number_with_format(row, 7, avg_unit, &highlight_money_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;
        sheet
            .write_number_with_format(row, 8, 0.0, &total_money_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;
        sheet
            .write_number_with_format(row, 9, 0.0, &total_money_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;

        let avg_value_rate = if sum_total_sales.abs() > f64::EPSILON {
            (sum_added_value / sum_total_sales) * 100.0
        } else {
            0.0
        };
        sheet
            .write_number_with_format(row, 10, avg_value_rate, &total_pct_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;

        let avg_expense_rate = if sum_total_sales.abs() > f64::EPSILON {
            (sum_total_expense / sum_total_sales) * 100.0
        } else {
            0.0
        };
        sheet
            .write_number_with_format(row, 11, avg_expense_rate, &total_pct_fmt)
            .map_err(|e| format!("写入合计行失败: {}", e))?;
    }

    Ok(())
}

/// Sheet2: 费用明细
fn write_expense_sheet(
    sheet: &mut Worksheet,
    records: &[AccountingRecord],
    amoeba: &Amoeba,
) -> Result<(), String> {
    let title_fmt = styles::title_format();
    let header_fmt = styles::header_format();
    let normal_fmt = styles::normal_format();
    let normal_left_fmt = styles::normal_left_format();
    let money_fmt = styles::money_format();
    let neg_money_fmt = styles::negative_money_format();

    // 设置列宽
    sheet
        .set_column_width(0, 14)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(1, 20)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(2, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(3, 30)
        .map_err(|e| format!("设置列宽失败: {}", e))?;

    // 标题行
    let title = format!("{} - 费用明细", amoeba.name);
    sheet
        .merge_range(0, 0, 0, 3, title.as_str(), &title_fmt)
        .map_err(|e| format!("写入标题失败: {}", e))?;

    // 表头行
    let headers = ["期间", "费用分类", "金额", "说明"];
    for (col, header) in headers.iter().enumerate() {
        sheet
            .write_string_with_format(1, col as u16, *header, &header_fmt)
            .map_err(|e| format!("写入表头失败: {}", e))?;
    }

    // 数据行
    let mut row = 2u32;
    for record in records {
        for expense in &record.expenses {
            sheet
                .write_string_with_format(row, 0, &record.period_start, &normal_fmt)
                .map_err(|e| format!("写入数据失败: {}", e))?;
            sheet
                .write_string_with_format(row, 1, &expense.category, &normal_left_fmt)
                .map_err(|e| format!("写入数据失败: {}", e))?;

            let fmt = if expense.amount < 0.0 { &neg_money_fmt } else { &money_fmt };
            sheet
                .write_number_with_format(row, 2, expense.amount, fmt)
                .map_err(|e| format!("写入数据失败: {}", e))?;
            sheet
                .write_string_with_format(row, 3, &expense.description, &normal_left_fmt)
                .map_err(|e| format!("写入数据失败: {}", e))?;

            row += 1;
        }
    }

    Ok(())
}

/// Sheet3: 趋势分析
fn write_trend_sheet(
    sheet: &mut Worksheet,
    records: &[AccountingRecord],
    amoeba: &Amoeba,
) -> Result<(), String> {
    let title_fmt = styles::title_format();
    let header_fmt = styles::header_format();
    let normal_fmt = styles::normal_format();
    let money_fmt = styles::money_format();
    let neg_money_fmt = styles::negative_money_format();
    let percent_fmt = styles::percent_format();
    let highlight_money_fmt = styles::highlight_money_format();

    // 设置列宽
    sheet
        .set_column_width(0, 14)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(1, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(2, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(3, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(4, 14)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(5, 18)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(6, 16)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(7, 14)
        .map_err(|e| format!("设置列宽失败: {}", e))?;
    sheet
        .set_column_width(8, 14)
        .map_err(|e| format!("设置列宽失败: {}", e))?;

    // 标题行
    let title = format!("{} - 趋势分析", amoeba.name);
    sheet
        .merge_range(0, 0, 0, 8, title.as_str(), &title_fmt)
        .map_err(|e| format!("写入标题失败: {}", e))?;

    // 表头行
    let headers = [
        "期间",
        "总销售额",
        "总费用",
        "附加价值",
        "总工时(h)",
        "单位时间附加值",
        "人均销售额",
        "附加值率(%)",
        "费用率(%)",
    ];
    for (col, header) in headers.iter().enumerate() {
        sheet
            .write_string_with_format(1, col as u16, *header, &header_fmt)
            .map_err(|e| format!("写入表头失败: {}", e))?;
    }

    // 数据行
    let mut row = 2u32;
    for record in records {
        let result = match &record.result {
            Some(r) => r,
            None => continue,
        };

        sheet
            .write_string_with_format(row, 0, &record.period_start, &normal_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 1, result.total_sales, &money_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;

        let exp_fmt = if result.total_expense < 0.0 { &neg_money_fmt } else { &money_fmt };
        sheet
            .write_number_with_format(row, 2, result.total_expense, exp_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;

        let av_fmt = if result.added_value < 0.0 { &neg_money_fmt } else { &money_fmt };
        sheet
            .write_number_with_format(row, 3, result.added_value, av_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 4, result.total_hours, &normal_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;

        let uv_fmt = if result.unit_value < 0.0 {
            &neg_money_fmt
        } else {
            &highlight_money_fmt
        };
        sheet
            .write_number_with_format(row, 5, result.unit_value, uv_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 6, result.sales_per_person, &money_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 7, result.value_rate, &percent_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;
        sheet
            .write_number_with_format(row, 8, result.expense_rate, &percent_fmt)
            .map_err(|e| format!("写入数据失败: {}", e))?;

        row += 1;
    }

    Ok(())
}
