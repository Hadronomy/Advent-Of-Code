{
  description = "Advent Of Code development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux"; # Adjust this to your system architecture
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          rust
          pkgs.cmake
          pkgs.pkg-config
          pkgs.z3
          pkgs.llvmPackages.libclang
          pkgs.nushell
        ];

        shellHook = ''
          export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"
          export Z3_INCLUDE_DIR="${pkgs.z3.dev}/include"
          export Z3_LIB_DIR="${pkgs.z3.dev}/lib"
          echo "Development environment for Rust with z3-sys and nushell is ready."
          exec nu
        '';
      };
    };
}
