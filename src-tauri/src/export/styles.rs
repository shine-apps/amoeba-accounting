use rust_xlsxwriter::{Format, FormatBorder, FormatAlign, Color};

/// 创建标题行样式 - 蓝色背景、白色粗体文字、居中
pub fn title_format() -> Format {
    Format::new()
        .set_bold()
        .set_font_color(Color::White)
        .set_background_color(Color::RGB(0x1F4E79))
        .set_border(FormatBorder::Thin)
        .set_text_wrap()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

/// 创建表头样式 - 浅蓝背景、深色粗体文字、居中
pub fn header_format() -> Format {
    Format::new()
        .set_bold()
        .set_font_color(Color::RGB(0x1F4E79))
        .set_background_color(Color::RGB(0xD6E4F0))
        .set_border(FormatBorder::Thin)
        .set_text_wrap()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

/// 创建普通单元格样式
pub fn normal_format() -> Format {
    Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

/// 创建左对齐单元格样式
pub fn normal_left_format() -> Format {
    Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Left)
        .set_align(FormatAlign::VerticalCenter)
}

/// 创建金额格式样式 - 千分位、两位小数
pub fn money_format() -> Format {
    Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Right)
        .set_align(FormatAlign::VerticalCenter)
        .set_num_format("#,##0.00")
}

/// 创建百分比格式样式
pub fn percent_format() -> Format {
    Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Right)
        .set_align(FormatAlign::VerticalCenter)
        .set_num_format("0.00\"%\"")
}

/// 创建合计行样式 - 浅绿背景、粗体
pub fn total_format() -> Format {
    Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0xE2EFDA))
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

/// 创建合计行金额格式
pub fn total_money_format() -> Format {
    Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0xE2EFDA))
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Right)
        .set_align(FormatAlign::VerticalCenter)
        .set_num_format("#,##0.00")
}

/// 创建合计行百分比格式
pub fn total_percent_format() -> Format {
    Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0xE2EFDA))
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Right)
        .set_align(FormatAlign::VerticalCenter)
        .set_num_format("0.00\"%\"")
}

/// 创建核心指标行样式 - 浅黄背景、粗体
pub fn highlight_format() -> Format {
    Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0xFFF2CC))
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

/// 创建核心指标行金额格式
pub fn highlight_money_format() -> Format {
    Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0xFFF2CC))
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Right)
        .set_align(FormatAlign::VerticalCenter)
        .set_num_format("#,##0.00")
}

/// 创建负数红色金额格式
pub fn negative_money_format() -> Format {
    Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Right)
        .set_align(FormatAlign::VerticalCenter)
        .set_num_format("#,##0.00;[Red]-#,##0.00")
}

/// 创建负数红色合计行金额格式
pub fn negative_total_money_format() -> Format {
    Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0xE2EFDA))
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Right)
        .set_align(FormatAlign::VerticalCenter)
        .set_num_format("#,##0.00;[Red]-#,##0.00")
}
