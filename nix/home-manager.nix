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
    enable = lib.mkEnableOption "Enable the osatui TUI client";

    config = lib.mkOption {
      type = lib.types.attrs;
      default = { };
      description = "osatui main config.toml options";
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
    };

    theme = lib.mkOption {
      type = lib.types.attrs;
      default = { };
      description = "osatui theme.toml options";
      example = {
        colors = {
          background = "#1e1e2e";
          text = "#cdd6f4";
          accent = "#89b4fa";
        };
      };
    };
  };

  config = lib.mkIf cfg.enable {

    home.packages = [ pkgs.osatui ];

    xdg.configFile."osatui/config.toml".text = lib.generators.toTOML { } cfg.config;
    xdg.configFile."osatui/theme.toml".text = lib.generators.toTOML { } cfg.theme;
  };
}
