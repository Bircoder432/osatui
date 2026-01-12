{
  config,
  pkgs,
  lib,
  ...
}:

let
  cfg = config.programs.osatui;
in
{
  options.programs.osatui = {
    enable = lib.mkEnableOption "Enable osatui terminal UI";

    package = lib.mkOption {
      type = lib.types.package;
      default = config.packages.osatui;
      description = "osatui package to use";
    };

    theme = lib.mkOption {
      type = lib.types.attrsOf lib.types.attrsOf lib.types.str;
      default = {
        dark = {
          background = "#1e1e1e";
          text = "#dcdcdc";
          header_bg = "#0064c8";
          header_fg = "#ffffff";
          table_header = "#ffff00";
          border = "#646464";
          highlight = "#00c800";
        };
      };
      description = "Theme colors for osatui";
    };

    config = lib.mkOption {
      type = lib.types.attrsOf lib.types.anything;
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

  config = lib.mkIf cfg.enable {
    # Устанавливаем бинарь
    home.packages = [ cfg.package ];

    # Генерация config.toml
    home.file.".config/osatui/config.toml".text = ''
      [api]
      url = "${cfg.config.api.url}"
      college_id = ${toString cfg.config.api.college_id}
      campus_id  = ${toString cfg.config.api.campus_id}
      group_id   = ${toString cfg.config.api.group_id}

      [app]
      refresh_interval = ${toString cfg.config.app.refresh_interval}
      cache_enabled    = ${toString cfg.config.app.cache_enabled}
      cache_ttl        = ${toString cfg.config.app.cache_ttl}
      current_theme    = "${cfg.config.app.current_theme}"

      [keymap]
      prev_day = "${cfg.config.keymap.prev_day}"
      cur_day  = "${cfg.config.keymap.cur_day}"
      next_day = "${cfg.config.keymap.next_day}"

      [keymap.selector]
      Char = "${cfg.config.keymap.selector.Char}"

      [keymap.settings]
      Char = "${cfg.config.keymap.settings.Char}"

      [keymap.exit]
      Char = "${cfg.config.keymap.exit.Char}"
    '';

    # Генерация theme.toml
    home.file.".config/osatui/theme.toml".text = ''
      [dark]
      background   = "${cfg.theme.dark.background}"
      text         = "${cfg.theme.dark.text}"
      header_bg    = "${cfg.theme.dark.header_bg}"
      header_fg    = "${cfg.theme.dark.header_fg}"
      table_header = "${cfg.theme.dark.table_header}"
      border       = "${cfg.theme.dark.border}"
      highlight    = "${cfg.theme.dark.highlight}"
    '';
  };
}
