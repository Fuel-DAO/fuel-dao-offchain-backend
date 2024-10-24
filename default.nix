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
          };
        };
      })
    ];
  };
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
      rustup
      binaryen
      pkg-config
      openssl
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
      file
      gnumake
      binutils
      binutils.bintools
    ] ++ (if pkgs.stdenv.isDarwin then [
      darwin.apple_sdk.frameworks.Foundation
      pkgs.darwin.libiconv
    ] else []);

  # Build dependencies for cross-compilation
  buildInputs = with pkgs; old.buildInputs ++ [
    openssl.dev
    muslPackages.stdenv.cc.libc
    zlib.dev
    zlib.static
  ];

  # Environment variables for cross-compilation
  NIX_LDFLAGS = "-L${pkgs.openssl.out}/lib -L${pkgs.zlib.static}/lib";

  # Shell hooks (executed when the shell starts)
  shellHook = ''
    # Create necessary directories
    mkdir -p $HOME/.cargo/bin
    export PATH="$HOME/.cargo/bin:$PATH"

    # Setup cross-compilation environment
    export CC_x86_64_unknown_linux_musl="${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc"
    export CXX_x86_64_unknown_linux_musl="${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-g++"
    export AR_x86_64_unknown_linux_musl="${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-ar"
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc"

    # OpenSSL configuration for cross-compilation
    export OPENSSL_DIR="${pkgs.openssl.dev}"
    export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
    export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
    export PKG_CONFIG_ALLOW_CROSS=1

    # Add the musl target for Rust
    rustup target add wasm32-unknown-unknown
    rustup target add x86_64-unknown-linux-musl
    rustup component add rustfmt
    rustup component add clippy

    # Create cargo config for cross-compilation
    mkdir -p ~/.cargo
    cat > ~/.cargo/config.toml << EOF
    [target.x86_64-unknown-linux-musl]
    linker = "x86_64-unknown-linux-musl-gcc"
    rustflags = [
      "-C", "target-feature=+crt-static",
      "-C", "link-arg=-static"
    ]
    EOF

    # Install candid-extractor in user's cargo directory instead of nix store
    if ! command -v candid-extractor &> /dev/null; then
      echo "Installing candid-extractor..."
      cargo install --quiet candid-extractor
    fi

    # Print versions
    echo "Node.js version: $(node -v)"
    echo "npm version: $(npm -v)"
    echo "Trunk version: $(trunk -V)"
    echo "GCC version: $(gcc --version | head -n1)"
    echo "Musl CC version: $(${pkgs.muslPackages.stdenv.cc}/bin/x86_64-unknown-linux-musl-gcc --version | head -n1)"

    # Print candid-extractor version if installed
    if command -v candid-extractor &> /dev/null; then
      echo "candid-extractor is installed in $(which candid-extractor)"
    else
      echo "Warning: candid-extractor installation failed"
    fi
  '';
})
