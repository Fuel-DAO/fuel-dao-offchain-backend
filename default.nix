{}:
let
  # The specific revision of nixpkgs we want to use
  rev = "1c3a28d84f970e7774af04372ade06399add182e";

  # Fetch the Nixpkgs repository
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";

  # Import Nixpkgs
  pkgs = import nixpkgs { };

  # Fetch the DFX environment
  dfx-env = import (builtins.fetchTarball "https://github.com/ninegua/ic-nix/releases/download/20240610/dfx-env.tar.gz") {
    version = "20240610";
    inherit pkgs;
  };
in
# Create the shell environment
dfx-env.overrideAttrs (old: {
  nativeBuildInputs = old.nativeBuildInputs ++ [
    pkgs.binaryen           # WebAssembly tools
    pkgs.flyctl             # Fly.io CLI tool
    pkgs.rustup             # For managing Rust toolchains
    pkgs.openssl            # Secure network connections
    pkgs.openssl.dev        # Development package for OpenSSL
    pkgs.pkg-config         # Package configuration tool
    pkgs.protobuf_21        # Protobuf library
  ] ++ (if pkgs.stdenv.isDarwin then [
    pkgs.darwin.apple_sdk.frameworks.Foundation
    pkgs.darwin.libiconv
  ] else []);

  shellHook = ''
    # Set environment variables for OpenSSL
    export OPENSSL_DIR="${pkgs.openssl}/lib"
    export PKG_CONFIG_PATH="${pkgs.pkg-config}/lib/pkgconfig:${pkgs.openssl}/lib/pkgconfig"

    # Add the wasm32 target to Rust
    rustup target add wasm32-unknown-unknown

    # Install candid-extractor
    cargo install --root $out --force candid-extractor
    ln -s $out/bin/candid-extractor $out/bin/candid-extractor

    # Add Node.js and npm binaries to PATH
    export PATH="$out/bin:$PATH"

    # Print installed versions
    echo "Binaryen version: $(binaryen --version)"
    echo "Flyctl version: $(flyctl --version)"
    echo "Rustup version: $(rustup --version)"
    echo "OpenSSL version: $(openssl version)"
    echo "Protobuf version: $(protoc --version)"
  '';
})

