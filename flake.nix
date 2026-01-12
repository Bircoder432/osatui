{
  description = "osatui – terminal UI for student schedules";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    home-manager.url = "github:nix-community/home-manager";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      naersk,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (final: prev: {
              osatui = final.callPackage ./nix/package.nix {
                naersk = naersk.lib.${system};
              };
            })
          ];
        };
      in
      {
        packages.default = pkgs.osatui;
        packages.osatui = pkgs.osatui;

        apps.default = {
          type = "app";
          program = "${pkgs.osatui}/bin/osatui";
        };

        homeManagerModules.osatui = import ./nix/home-manager.nix;
      }
    );
}
