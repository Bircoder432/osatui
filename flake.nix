{
  description = "osatui Rust app + Home Manager module";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    home-manager.url = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      naersk,
      home-manager,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        nm = home-manager.lib.homeManagerConfiguration;
      in
      {
        # Rust package через naersk
        defaultPackage = pkgs.callPackage naersk { }.buildPackage {
          src = ./.;
        };

        # devShell для разработки
        devShell = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.pkg-config
            pkgs.openssl
          ];
        };

        homeManagerModules.osatui = ./nix/home-manager.nix;
      }
    );
}
