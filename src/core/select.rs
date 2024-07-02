use anyhow::{ Result, Context };
use console::Style;
use dialoguer::{ console::{ style, Term }, theme::ColorfulTheme, Select };

pub fn create_list(items: &[&str], default: usize) -> Result<usize> {
    Select::with_theme(
        &(ColorfulTheme {
            active_item_prefix: style("❯".to_string()).for_stderr().color256(69),
            active_item_style: Style::new().for_stderr().color256(69),
            ..ColorfulTheme::default()
        })
    )
        .items(&items)
        .default(default)
        .interact_on_opt(&Term::stderr())
        .context("选择项失败")?
        .ok_or_else(|| anyhow::anyhow!("未选择任何项"))
}
