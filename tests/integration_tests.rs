use osatui::{config::Config, utils::AppDate};

#[tokio::test]
async fn test_config_default() {
    let config = Config::default();
    assert!(!config.api_url().is_empty());
    assert!(config.cache_enabled());
}

#[test]
fn test_app_date_navigation() {
    let today = AppDate::today();
    let yesterday = today.prev();
    let tomorrow = today.next();

    // Verify navigation works
    assert_ne!(today, yesterday);
    assert_ne!(today, tomorrow);

    // Verify roundtrip
    assert_eq!(yesterday.next(), today);
    assert_eq!(tomorrow.prev(), today);
}

#[test]
fn test_app_date_formatting() {
    let today = AppDate::today();
    let formatted = today.format();

    // Should contain "Today" or a date
    assert!(!formatted.is_empty());
}
