{ }:
let
  # Specific revision of nixpkgs
  rev = "1c3a28d84f970e7774af04372ade06399add182e";

  # Fetch the Nixpkgs repository
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";

  # Import Nixpkgs
  pkgs = import nixpkgs { };

  # Fetch and import the DFX environment for Internet Computer development
  dfx-env = import (builtins.fetchTarball "https://github.com/ninegua/ic-nix/releases/download/20240610/dfx-env.tar.gz") {
    version = "20240610";
    inherit pkgs;
  };
in
# Override the attributes of the DFX environment
dfx-env.overrideAttrs (old: {
  # Adding native build inputs (tools and libraries we want available)
  nativeBuildInputs = with pkgs; old.nativeBuildInputs ++
    [
      rustup              # For managing Rust toolchains
      binaryen
      pkg-config          # For managing build configurations
      openssl
      openssl.dev         # OpenSSL development libraries
      protobuf            # For working with Protocol Buffers
      protobuf_21
      cmake               # Build system
      cachix              # Caching for build artifacts
      killall             # Unix utility for killing processes
      jq                  # Command-line JSON processor
      coreutils           # Basic GNU utilities (ls, cat, etc.)
      bc                  # Command-line calculator
      python3Full         # Full Python 3 environment
      libiconv            # Text conversion library
      wget                # Tool to download files from the web
      nodejs              # Node.js runtime (includes npm)
      trunk               # Trunk for managing front-end assets
      musl                # musl for cross-compiling
      gcc                 # C Compiler
    ] ++ (if pkgs.stdenv.isDarwin then [
      darwin.apple_sdk.frameworks.Foundation
      pkgs.darwin.libiconv
    ] else []);

  # Build dependencies for cross-compilation
  buildInputs = with pkgs; old.buildInputs ++ [
    openssl.dev  # OpenSSL for cross-compilation
  ];

  # Shell hooks (executed when the shell starts)
  shellHook = ''
      # Add the musl target for Rust
      rustup target add wasm32-unknown-unknown
      rustup target add x86_64-unknown-linux-musl
      rustup component add rustfmt
      rustup component add clippy

      # Install candid-extractor
      cargo install --root $out --force candid-extractor
      ln -s $out/bin/candid-extractor $out/bin/candid-extractor


      # Add Node.js and npm binaries to PATH
      export PATH="$out/bin:$PATH"

      # Print versions
      echo "Node.js version: $(node -v)"
      echo "npm version: $(npm -v)"
      echo "Trunk version: $(trunk -V)"
    '';
})

