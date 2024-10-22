{
  let
    # The specific revision of nixpkgs we want to use
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
    # Adding native build inputs (only the required tools)
    nativeBuildInputs = with pkgs; old.nativeBuildInputs ++
      [
        binaryen           # WebAssembly tools
        flyctl             # Fly.io CLI tool
        rustup             # For managing Rust toolchains
        openssl            # Secure network connections
        protobuf_21        # Protocol Buffers version 21
      ];

    # Shell hooks (executed when the shell starts)
    shellHook = ''
      # Add the wasm32 target to Rust
      rustup target add wasm32-unknown-unknown

      # Print installed versions to verify installation
      echo "Binaryen version: $(binaryen --version)"
      echo "Flyctl version: $(flyctl --version)"
      echo "Rustup version: $(rustup --version)"
      echo "OpenSSL version: $(openssl version)"
      echo "Protobuf version: $(protoc --version)"
    '';
  })
}

