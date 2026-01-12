{
  lib,
  pkgs,
  config,
  ...
}:

let
  osatuiCfg = config.osatui;
in
{
  options.osatui = {
    enable = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Enable osatui configuration";
    };

    theme = lib.mkOption {
      type = lib.types.attrsOf lib.types.str;
      default = {
        background = "#1e1e1e";
        text = "#dcdcdc";
        header_bg = "#0064c8";
        header_fg = "#ffffff";
        table_header = "#ffff00";
        border = "#646464";
        highlight = "#00c800";
      };
      description = "Theme colors for osatui, can override any color";
    };

    config = lib.mkOption {
      type = lib.types.attrsOf lib.types.any;
      default = {
        api = {
          url = "https://api.example.com";
          college_id = 1;
          campus_id = 1;
          group_id = 1;
        };
        app = {
          refresh_interval = 300;
          cache_enabled = true;
          cache_ttl = 3600;
          current_theme = "dark";
        };
        keymap = {
          prev_day = "Left";
          cur_day = "Up";
          next_day = "Right";
          selector = {
            Char = "o";
          };
          settings = {
            Char = "s";
          };
          exit = {
            Char = "q";
          };
        };
      };
      description = "Application settings for osatui";
    };
  };

  config = lib.mkIf osatuiCfg.enable {
    home.file.".config/osatui/config.toml".text = lib.toToml osatuiCfg.config;
    home.file.".config/osatui/theme.toml".text = lib.toToml { dark = osatuiCfg.theme; };
  };
}
