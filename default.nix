{ }:
let
  # Specific revision of nixpkgs
  rev = "1c3a28d84f970e7774af04372ade06399add182e";
  # Fetch the Nixpkgs repository
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
  # Import Nixpkgs with overlay for musl
  pkgs = import nixpkgs {
    overlays = [
      (self: super: {
        muslPackages = import nixpkgs {
          localSystem = "x86_64-linux";
          crossSystem = {
            config = "x86_64-unknown-linux-musl";
            libc = "musl";
            useLLVM = false;  # Ensure we use GCC
          };
        };
      })
    ];
  };
  # Fetch and import the DFX environment
  dfx-env = import (builtins.fetchTarball "https://github.com/ninegua/ic-nix/releases/download/20240610/dfx-env.tar.gz") {
    version = "20240610";
    inherit pkgs;
  };
in
dfx-env.overrideAttrs (old: {
  nativeBuildInputs = with pkgs; old.nativeBuildInputs ++
    [
      rustup
      binaryen
      pkg-config
      openssl.dev
      protobuf
      protobuf_21
      cmake
      cachix
      killall
      jq
      coreutils
      bc
      python3Full
      libiconv
      wget
      nodejs
      trunk
      musl
      gcc
      muslPackages.stdenv.cc
      muslPackages.pkgsStatic.openssl
      muslPackages.pkgsStatic.zlib
      pkgs.pkgsStatic.openssl
      file
      gnumake
      binutils
      binutils.bintools
      pkgs.stdenv.cc.cc.lib
    ] ++ (if pkgs.stdenv.isDarwin then [
      darwin.apple_sdk.frameworks.Foundation
      pkgs.darwin.libiconv
    ] else []);

  buildInputs = with pkgs; old.buildInputs ++ [
    openssl.dev
    muslPackages.stdenv.cc.libc
    muslPackages.pkgsStatic.openssl
    muslPackages.pkgsStatic.zlib
    pkgs.pkgsStatic.openssl.dev
    pkgs.pkgsStatic.openssl.out
    zlib.dev
    zlib.static
    stdenv.cc.cc.lib
  ];

  shellHook = ''
    # Setup directories
    mkdir -p $HOME/.cargo/bin
    export PATH="$HOME/.cargo/bin:$PATH"

    # Enhanced musl configuration
    export TARGET_CC="${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc"
    export TARGET_AR="${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-ar"
    export CC_x86_64_unknown_linux_musl="${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc"
    export CXX_x86_64_unknown_linux_musl="${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-g++"
    export AR_x86_64_unknown_linux_musl="${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-ar"
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc"

      # Add these lines at the beginning of your shellHook
  export CARGO_BUILD_TARGET="x86_64-unknown-linux-musl"
  export CARGO_TARGET_DIR="target"
  export HOST_CC="gcc"
  export CC="gcc"

  # Modify your existing RUSTFLAGS
  export RUSTFLAGS="-C target-feature=+crt-static -C linker=${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc"

  # Add these environment variables
  export CARGO_BUILD_RUSTFLAGS="$RUSTFLAGS"
  export CARGO_ENCODED_RUSTFLAGS="$RUSTFLAGS"

    # Static linking configuration
    export RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-static"
    export OPENSSL_STATIC=1
    export OPENSSL_DIR="${pkgs.pkgsStatic.openssl.dev}"
    export OPENSSL_LIB_DIR="${pkgs.pkgsStatic.openssl.out}/lib"
    export OPENSSL_INCLUDE_DIR="${pkgs.pkgsStatic.openssl.dev}/include"
    export PKG_CONFIG_PATH="${pkgs.pkgsStatic.openssl.dev}/lib/pkgconfig"
    export PKG_CONFIG_ALLOW_CROSS=1
    export PKG_CONFIG_ALL_STATIC=1

    # Library path setup
    export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.zlib.out}/lib:$LD_LIBRARY_PATH"

    # Rust toolchain setup
    rustup toolchain install stable
    rustup default stable
    rustup target add wasm32-unknown-unknown
    rustup target add x86_64-unknown-linux-musl
    rustup component add rustfmt
    rustup component add clippy

    # Enhanced cargo config

    [target.x86_64-unknown-linux-musl]
    rustflags = [
      "-C", "target-feature=+crt-static",
      "-C", "link-arg=-static",
      "-C", "link-arg=-s",
      "-C", "link-arg=-lgcc"
    ]
    linker = "${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc"
    ar = "${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-ar"
    EOF

    # Install candid-extractor
    if ! command -v candid-extractor &> /dev/null; then
      cargo install --quiet candid-extractor
    fi

    # Print versions and verify setup
    echo "Node.js version: $(node -v)"
    echo "npm version: $(npm -v)"
    echo "Trunk version: $(trunk -V)"
    echo "GCC version: $(gcc --version | head -n1)"
    echo "Rust version: $(rustc --version)"
    echo "Musl GCC version: $(${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc --version | head -n1)"
    echo "OpenSSL static lib path: $OPENSSL_LIB_DIR"
    echo "OpenSSL include path: $OPENSSL_INCLUDE_DIR"
  '';
})
