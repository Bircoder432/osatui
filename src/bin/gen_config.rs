#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = osatui::Config::default();
    let theme = osatui::config::theme::Theme::default();

    let config_toml = toml::to_string_pretty(&config)?;
    let theme_toml = toml::to_string_pretty(&theme)?;

    tokio::fs::write("config.toml.example", config_toml).await?;
    tokio::fs::write("theme.toml.example", theme_toml).await?;

    println!("Generated config.toml.example and theme.toml.example");
    Ok(())
}
