{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.services.osatui;
in
{
  options.services.osatui = {
    enable = lib.mkEnableOption "osatui TUI client";

    config = lib.mkOption {
      type = lib.types.attrs;
      default = { };
      description = "System-wide osatui config.toml";
    };

    theme = lib.mkOption {
      type = lib.types.attrs;
      default = { };
      description = "System-wide osatui theme.toml";
    };

    users = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      example = [
        "alice"
        "bob"
      ];
      description = "Users allowed to read system osatui config";
    };
  };

  config = lib.mkIf cfg.enable {

    environment.systemPackages = [ pkgs.osatui ];

    environment.etc."osatui/config.toml".text = lib.generators.toTOML { } cfg.config;

    environment.etc."osatui/theme.toml".text = lib.generators.toTOML { } cfg.theme;

    users.users = lib.genAttrs cfg.users (_: {
      extraGroups = [ "osatui" ];
    });

    users.groups.osatui = { };
  };
}
