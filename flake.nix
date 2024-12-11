{
  description = "Blisk on NixOS";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:      
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
        libPath = with pkgs; lib.makeLibraryPath [
          # load external libraries
        ];
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            clang
            llvmPackages_19.bintools
            rustup
          ];
          RUSTC_VERSION = overrides.toolchain.channel;
          # https://github.com/rust-lang/rust-bindgen#environment-variables
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_19.libclang.lib ];
          shellHook = ''
            export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
            export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
            '';
          # Add precompiled library to rustc search path
          RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
            # add libraries here (e.g. pkgs.libvmi)
          ]);
          LD_LIBRARY_PATH = libPath;
          NIX_LD_LIBRARY_PATH = lib.makeLibraryPath [
            stdenv.cc.cc
            openssl
          ];
          NIX_LD = lib.fileContents "${stdenv.cc}/nix-support/dynamic-linker";
          # Add glibc, clang, glib, and other headers to bindgen search path
          BINDGEN_EXTRA_CLANG_ARGS =
          # Includes normal include path
          (builtins.map (a: ''-I"${a}/include"'') [
            # add dev libraries here (e.g. pkgs.libvmi.dev)
            pkgs.glibc.dev
          ])
          # Includes with special directory paths
          ++ [
            ''-I"${pkgs.llvmPackages_19.libclang.lib}/lib/clang/${pkgs.llvmPackages_19.libclang.version}/include"''
            ''-I"${pkgs.glib.dev}/include/glib-2.0"''
            ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
          ];
          packages = with pkgs; [
            stdenv
            openssl
            pkg-config
            rust-analyzer
            corepack_22
            nodejs_22
          ];
        };
      }
    );
}