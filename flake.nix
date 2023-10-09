{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, flake-parts, rust-overlay, crane, nixpkgs, ... }:
    flake-parts.lib.mkFlake {inherit inputs;} {
        systems = [
          "x86_64-linux"
          "aarch64-linux"
        ];

        perSystem = { pkgs, system, ... }: 
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          common = {
            nativeBuildInputs = with pkgs; [
              pkg-config
              openssl
              mold
            ];

            buildInputs = with pkgs; [
              libxkbcommon
              vulkan-loader

              wayland
              libGL

              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              xorg.libxcb
              xorg.libX11
            ];

            NIX_CFLAGS_LINK = "-fuse-ld=mold";
          };
                    
          craneLib = (crane.mkLib pkgs).overrideToolchain pkgs.rust-bin.stable.latest.default;
          cargoArtifacts = craneLib.buildDepsOnly common;

          phx = craneLib.buildPackage (common // {
            src = craneLib.cleanCargoSource (craneLib.path ./.);
            inherit cargoArtifacts;

            postFixup = with pkgs; ''
              patchelf --set-rpath "${
                pkgs.lib.makeLibraryPath (common.buildInputs)
              }" \
              $out/bin/crustility
            '';
          });
        in {

          packages = {
            inherit phx;
            default = phx;
          };

          devShells.default = pkgs.mkShell
            (common // {
              packages = with pkgs; [
                (rust-bin.stable.latest.default.override { extensions = [
                  "cargo"
                  "clippy"
                  "rust-src"
                  "rust-analyzer"
                  "rustc"
                  "rustfmt"
                ];})
              ];
              
              LD_LIBRARY_PATH=pkgs.lib.makeLibraryPath (common.buildInputs);
            });

        };
    };
}
