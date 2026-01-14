{
  config,
  pkgs,
  lib,
  ...
}:

let
  inherit (lib)
    mkEnableOption
    mkPackageOption
    mkOption
    literalExpression
    ;

  cfg = config.programs.osatui;
  tomlFormat = pkgs.formats.toml { };

  # Дефолтная тема
  defaultTheme = {
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

  defaultSettings = {
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

  getTheme =
    theme:
    if theme == null then
      null
    else if theme == "default" then
      defaultTheme
    else
      theme;

in
{
  options.programs.osatui = {
    enable = mkEnableOption "Enable osatui terminal UI";

    package = mkPackageOption pkgs "osatui" { nullable = true; };

    theme = mkOption {
      type = tomlFormat.type;
      default = null;
      example = literalExpression ''
        {
          dark = {
            background = "#000000";
            text = "#ffffff";
          }
      '';
      description = "Theme colors for osatui. Set null to disable generation, or 'default' to use the built-in default theme.";
    };

    settings = mkOption {
      type = tomlFormat.type;
      default = defaultSettings;
      example = literalExpression ''
        {
          api = { url = "https://api.example.com"; college_id = 2; campus_id = 2; group_id = 3; }
          app = { refresh_interval = 600; cache_enabled = false; }
      '';
      description = "Application settings for osatui";
    };
  };

  config = lib.mkIf cfg.enable {

    home.packages = lib.mkIf (cfg.package != null) [ cfg.package ];

    xdg.configFile."osatui/config.toml" = {
      source = tomlFormat.generate "config.toml" (
        lib.recursiveUpdate defaultSettings (cfg.settings or { })
      );
    };

    home.file = lib.optionalAttrs (getTheme cfg.theme != null) ({
      ".config/osatui/theme.toml".text = tomlFormat.generate "theme.toml" (
        lib.recursiveUpdate defaultTheme (if cfg.theme == "default" then { } else cfg.theme)
      );
    });
  };
}
