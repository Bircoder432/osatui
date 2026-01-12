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
    enable = lib.mkEnableOption "osatui terminal UI";

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

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];

    home.file.".config/osatui/config.toml".text = lib.toToml cfg.config;
    home.file.".config/osatui/theme.toml".text = lib.toToml cfg.theme;
  };
}
