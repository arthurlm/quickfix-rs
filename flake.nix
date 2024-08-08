{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    ,
    }:
    let
      buildSystemConfig = system: additionalNativeBuildInputs: (
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          rustToolchain = pkgs.rust-bin.fromRustupToolchain {
            channel = "stable";
            components = [ "rust-src" "rust-analyzer" "rustfmt" "rustc" "clippy" "cargo" "rust-docs" ];
          };
        in
        {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              rustPlatform.bindgenHook
              pkgs.rust-analyzer-unwrapped
              protobuf3_23
              cmake
              pkg-config
              wireshark
              openssl # dependency of reqwest
              nodejs_22 # needed for the web client

              # for the quickfix client
              maven
              jdk11

              # Aeron
              aeron

              # caching rust builds
              sccache
            ] ++ additionalNativeBuildInputs pkgs;

            packages = [
              rustToolchain
              pkgs.sqlx-cli
              pkgs.lldb
              pkgs.cargo-expand
              pkgs.python3Packages.tomli
            ];

            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            WIRESHARK_LIB_DIR = "${pkgs.wireshark}/lib/wireshark";

            RUST_BACKTRACE = "full";
            RUST_LOG = "INFO";
            GIT_NET_FETCH_WITH_CLI = "true";
            RE_CHANNEL = "aeron:ipc";
            ME_CHANNEL = "aeron:ipc";
            INIT_TESTING_STATE = "true";

            RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";

            AERON_DIR="/tmp/aeronmd/aeron";
            ARCHIVE_DIR="/tmp/aeronmd/aeron-archive";

            CARGO_NET_GIT_FETCH_WITH_CLI = "true";

          };
        }
      );
    in
    {
      devShells.x86_64-linux = buildSystemConfig "x86_64-linux" (pkgs: [ ]);
      devShells.aarch64-darwin = buildSystemConfig "aarch64-darwin" (pkgs: [ pkgs.darwin.apple_sdk.frameworks.SystemConfiguration ]);
    };
}
