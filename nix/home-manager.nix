{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.osatui;
in
{
  options.programs.osatui = {
    enable = lib.mkEnableOption "osatui TUI client";

    config = lib.mkOption {
      type = lib.types.attrs;
      default = { };
      example = {
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
      };
      description = "osatui main config.toml";
    };

    theme = lib.mkOption {
      type = lib.types.attrs;
      default = { };
      example = {
        colors = {
          background = "#1e1e2e";
          text = "#cdd6f4";
          accent = "#89b4fa";
        };
      };
      description = "osatui theme.toml";
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ pkgs.osatui ];

    xdg.configFile."osatui/config.toml".text = lib.generators.toTOML { } cfg.config;

    xdg.configFile."osatui/theme.toml".text = lib.generators.toTOML { } cfg.theme;
  };
}
