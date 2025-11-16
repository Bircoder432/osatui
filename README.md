# osatui
![screenshot](https://github.com/bircoder432/osatui/raw/master/.assets/screenshot.png)
Terminal UI client for educational schedule API with interactive college, campus and group selection.

## Features

- Interactive selection of colleges, campuses and groups
- Daily schedule viewing with navigation
- Customizable themes
- Local caching for faster loading
- Keyboard-driven interface
- Automatic configuration setup

## Installation

`cargo install osatui`

## Usage

`osatui`

### Keybindings

**Normal Mode:**
- F1 - Previous day
- F2 - Today
- F3 - Next day
- Ctrl+O - Open selector to change group
- Ctrl+S - Open settings
- Q - Quit

**Selector Mode:**
- ↑/↓ - Navigate items
- ←/→ - Change pages
- Enter - Select item
- Esc - Cancel

## Configuration

Configuration files are automatically created in system config directory.

### Example config.toml
```toml
[api]
url = "https://api.example.com"
college_id = 1
campus_id = 1
group_id = 1

[app]
refresh_interval = 300
cache_enabled = true
cache_ttl = 3600
current_theme = "dark"
```
## Related Projects

This project is built on top of:

- [OpenScheduleApi](https://github.com/thisishyum/OpenScheduleApi) - The backend API providing educational schedule data
- [osars](https://github.com/bircoder432/osars) - Rust client library for OpenScheduleApi


## License

MIT
