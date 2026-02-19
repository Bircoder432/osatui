 # osatui
![screenshot](https://github.com/bircoder432/osatui/raw/master/.assets/screenshot.png)

Terminal UI client for educational schedule API with interactive college, campus and group selection.

## Features

- Interactive selection of colleges, campuses and groups
- Daily schedule viewing with navigation
- Customizable themes
- Local caching per group for faster loading
- Keyboard-driven interface
- Automatic configuration setup

## Installation

**via cargo**
```bash
cargo install osatui
```

**via nix**

*flake.nix*
```nix
{
  inputs = {
    osatui.url = "github:Bircoder432/osatui"
  };

  outputs = {self, ... }@inputs:
  {
    nixosConfigurations = {
      your = nixpkgs.lib.nixosSystem {
        specialArgs = { inherit inputs; };
        modules = [
          # ...
          home-manager.nixosModules.home-manager
          {
            home-manager.extraSpecialArgs = { inherit inputs; };
          }
        ];
      };
    };
  };
}
```

*home-manager*
```nix
{ config, pkgs, inputs, ... }:

{
  imports = [ inputs.osatui.homeManagerModules.your-system.osatui ];
  programs.osatui.enable = true;
  programs.osatui.package = inputs.osatui.packages.${pkgs.system}.osatui;
  programs.osatui.settings = {
    api = {
      url = "https://api.thisishyum.ru/schedule_api/tyumen"; # Your OpenScheduleApi provider
      college_id = 1; # ID of your college in api
      campus_id = 1; # ID of your campus
      group_id = 161; # ID of your group
    };
    keymap = {
      prev_day = "Left"; # Key for move to previous day
      cur_day = "Up"; # Key for move to current day
      next_day = "Right"; # Key for move to next day
    };
  };
}
```

## Usage

```bash
osatui
```

### Keybindings

**Normal Mode:**
- ← - Previous day
- ↑ - Today
- → - Next day
- Ctrl+O - Open selector to change group
- Ctrl+S - Open settings
- Q - Quit
- Shift+R - Reload cache

**Selector Mode:**
- ↑/↓ - Navigate items
- ←/→ - Change pages
- Enter - Select item
- Esc - Cancel

**Settings Mode:**
- Tab - Next field
- Shift+Tab - Previous field
- Enter - Save
- Esc - Cancel

## Configuration

Configuration files are automatically created in system config directory.

### Example config.toml
```toml
[api]
url = "https://api.thisishyum.ru/schedule_api/tyumen"
college_id = 1
campus_id = 1
group_id = 161

[app]
refresh_interval = 300
cache_enabled = true
cache_ttl = 3600
current_theme = "dark"

[keymap]
prev_day = "Left"
cur_day = "Up"
next_day = "Right"
selector = "o"
settings = "s"
exit = "q"
```

### Example theme.toml
```toml
[dark]
background = "#1e1e1e"
text = "#dcdcdc"
header_bg = "#0064c8"
header_fg = "#ffffff"
table_header = "#ffff00"
border = "#646464"
highlight = "#00c800"
error = "#ff0000"
```

## Cache

osatui stores cache in `~/.cache/osatui/` with format `{group_id}-{date}.json`. Each group has isolated cache that automatically clears when switching groups.

## Related Projects

- [OpenScheduleApi](https://github.com/thisishyum/OpenScheduleApi) - The backend API providing educational schedule data
- [osars](https://github.com/bircoder432/osars) - Rust client library for OpenScheduleApi

## License

MIT
