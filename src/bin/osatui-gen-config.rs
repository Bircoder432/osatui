use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use osatui::config::{main::MainConfig, theme::ThemeConfig};
use tokio::fs;
use toml;

#[tokio::main]
async fn main() {
    let example_main_config = toml::to_string_pretty(&MainConfig::default()).unwrap();
    let example_theme_config = toml::to_string_pretty(&ThemeConfig::default()).unwrap();
    let example_main_file = PathBuf::from_str("config.toml.example").unwrap();
    let example_theme_file = PathBuf::from_str("theme.toml.example").unwrap();
    fs::write(example_main_file, example_main_config)
        .await
        .unwrap();
    fs::write(example_theme_file, example_theme_config)
        .await
        .unwrap();
}
