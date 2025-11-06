{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      naersk,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
          targets = [
            "x86_64-unknown-linux-gnu"
            "x86_64-unknown-linux-musl"
          ];
        };

        naersk-lib = pkgs.callPackage naersk {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [ rustToolchain ];
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        checks = {
          build = self.packages.${system}.default;
        };

        packages.default = naersk-lib.buildPackage {
          src = ./.;

          buildInputs = with pkgs; [
            openssl
            glibc
          ];
        };
      }
    );
}
