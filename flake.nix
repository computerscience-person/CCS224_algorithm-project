{
  description = "Cross compiling a rust program for windows";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, crane, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        toolchain = with fenix.packages.${system};
          combine [
            complete.rustc complete.cargo complete.clippy complete.rustfmt
            targets.x86_64-pc-windows-gnu.latest.rust-std
          ];

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        src = craneLib.cleanCargoSource (craneLib.path ./.);

        app-windows = craneLib.buildPackage {
          inherit src;
          
          strictDeps = true;
          doCheck = false;

          CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";

          depsBuildBuild = with pkgs; [
            pkgsCross.mingwW64.stdenv.cc
            pkgsCross.mingwW64.windows.pthreads
          ];

          outname = "fuzzy_file_searcher-windows";
        };

        app-linux = craneLib.buildPackage {
          inherit src;
          stricDeps = true;
        };
      in
      {
        packages = {
          windows = app-windows;
          default = app-linux;
          builder-windows = pkgs.writeShellApplication {
            name = "build-windows";
            text = ''
              nix build .#windows --out-link result-windows
            '';
          };
        };

        checks = {
          my-crate = app-windows;
        };

        devShells = {
          default = craneLib.devShell { };
        };
      }
    );
}

