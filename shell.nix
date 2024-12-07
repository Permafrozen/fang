{ pkgs ? import <nixpkgs> { } }:
let

  # --- Nodejs ---
  lib = import <nixpkgs/lib>;
  buildNodeJs =
    pkgs.callPackage "${<nixpkgs>}/pkgs/development/web/nodejs/nodejs.nix" {
      python = pkgs.python3;
    };

  nodejsVersion = lib.fileContents ./.nvmrc;

  nodejs = buildNodeJs {
    enableNpm = false;
    version = nodejsVersion;
    sha256 = "1a0zj505nhpfcj19qvjy2hvc5a7gadykv51y0rc6032qhzzsgca2";
  };

  NPM_CONFIG_PREFIX = toString ./npm_config_prefix;

  # --- Rustup ---
  overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
  libPath = with pkgs;
    lib.makeLibraryPath [
      # load external libraries that you need in your rust project here
    ];

in pkgs.mkShell rec {

  # --- Tauri  ---
  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    cargo
    cargo-tauri
    nodejs
  ];

  buildInputs = with pkgs; [
    # --- Tauri  ---
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl

    # --- Rustup ---
    clang
    llvmPackages_19.bintools
    rustup
  ];

  # --- Nodejs ---
  packages = with pkgs; [ nodejs nodePackages.npm ];

  inherit NPM_CONFIG_PREFIX;

  # --- Rustup ---
  RUSTC_VERSION =
    overrides.toolchain.channel; # https://github.com/rust-lang/rust-bindgen#environment-variables
  LIBCLANG_PATH =
    pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
    export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
    export PATH="${NPM_CONFIG_PREFIX}/bin:$PATH"
  '';
  # Add precompiled library to rustc search path
  RUSTFLAGS = (builtins.map (a: "-L ${a}/lib") [
    # add libraries here (e.g. pkgs.libvmi)
  ]);
  LD_LIBRARY_PATH =
    libPath; # Add glibc, clang, glib, and other headers to bindgen search path
  BINDGEN_EXTRA_CLANG_ARGS =
    # Includes normal include path
    (builtins.map (a: ''-I"${a}/include"'') [
      # add dev libraries here (e.g. pkgs.libvmi.dev)
      pkgs.glibc.dev
    ])
    # Includes with special directory paths
    ++ [
      ''
        -I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
      ''-I"${pkgs.glib.dev}/include/glib-2.0"''
      "-I${pkgs.glib.out}/lib/glib-2.0/include/"
    ];
}
