name: Build and Check linting

on:
  workflow_call:
    inputs:
      publish-artifact:
        default: false
        required: false
        type: boolean
  workflow_dispatch:
  pull_request:
    branches:
      - main

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    name: Build Check
    runs-on: ubuntu-latest

    env:
      BACKEND: ${{ secrets.BACKEND }}
      RUST_LOG: ${{ secrets.RUST_LOG }}
      SERVER_PORT: ${{ secrets.SERVER_PORT }}
      EMAIL_CLIENT_ID: ${{ secrets.EMAIL_CLIENT_ID }}
      EMAIL_CLIENT_SECRET: ${{ secrets.EMAIL_CLIENT_SECRET }}
      EMAIL_ACCESS_TOKEN: ${{ secrets.EMAIL_ACCESS_TOKEN }}
      EMAIL_REFRESH_TOKEN: ${{ secrets.EMAIL_REFRESH_TOKEN }}

    steps:
      - name: Checkout Code
        uses: actions/checkout@v3

      - name: Cache install Nix packages
        uses: rikhuijzer/cache-install@v1.1.4
        with:
          key: nix-${{ hashFiles('default.nix') }}

      - name: Cache Rust dependencies, build output, and DFX build cache
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: rust-test-${{ hashFiles('**/Cargo.lock') }}

      - name: Install prerequisites
        run: |
          sudo apt-get update
          sudo apt-get install -y protobuf-compiler musl-tools libssl-dev

      - name: Rust Setup
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: "stable"
          targets: "x86_64-unknown-linux-musl"

      - name: Build setup
        run: |
          rustup target add x86_64-unknown-linux-musl
          rustup component add rustfmt
          rustup component add clippy

      - name: Set up pkg-config for cross-compilation
        run: |
          echo 'export PKG_CONFIG="/usr/bin/x86_64-linux-musl-gcc -pkg-config"' >> $GITHUB_ENV
          echo 'export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-musl/pkgconfig"' >> $GITHUB_ENV

      - name: Build
        run: cargo build --release --target x86_64-unknown-linux-musl --verbose
        env:
          TARGET_CC: x86_64-linux-musl-gcc
          OPENSSL_DIR: /usr/include  # Adjust if necessary

      - run: touch .empty

      - name: Archive production artifacts
        uses: actions/upload-artifact@v4
        if: ${{ inputs.publish-artifact }}
        with:
          name: build-musl
          path: |
            target/x86_64-unknown-linux-musl/release/fuel-dao-off-chain-backend
            .empty

