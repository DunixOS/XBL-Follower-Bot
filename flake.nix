{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };
  outputs =
    {
      nixpkgs,
      naersk,
      flake-utils,
      treefmt-nix,
      rust-overlay,
      ...
    }:
    let
      overlays = {
        default = final: prev: {
          xfb =
            (final.callPackage naersk {
              cargo = final.rust-bin.nightly.latest.default;
              rustc = final.rust-bin.nightly.latest.default;
            }).buildPackage
              {
                pname = "xfb";
                src = ./.;
                buildInputs = with final; [
                  pkg-config
                ];
              };
        };
      };
    in
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
            overlays.default
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            nil
            nixd
            typos
            rustfmt
            pkg-config
            rust-analyzer
            rust-bin.nightly.latest.complete
          ];

          runtimeLibs =
            with pkgs;
            lib.optionals stdenv.isLinux [
              pkg-config
              wayland
            ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
        };

        packages.default = pkgs.xfb;
        formatter =
          (treefmt-nix.lib.evalModule pkgs (_: {
            projectRootFile = "flake.nix";
            programs = {
              nixfmt.enable = true;
              nixf-diagnose.enable = true;
              rustfmt.enable = true;
              toml-sort.enable = true;
            };
            settings.formatter.rustfmt = {
              unstable-features = true;
              tab_spaces = 2;
              trailing_semicolon = false;
              style_edition = "2024";
              use_try_shorthand = true;
              wrap_comments = true;
            };
          })).config.build.wrapper;
      }
    )
    // {
      inherit overlays;
    };
}
