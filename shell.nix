let pkgs = import <nixpkgs> { };
in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    xorg.libXtst
    rustc
    cargo
    cargo-tauri
    nodejs
  ];

  buildInputs = with pkgs; [
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
  ];

  XDG_DATA_DIRS = let
    base = pkgs.lib.concatMapStringsSep ":" (x: "${x}/share") [
      pkgs.adwaita-icon-theme
      pkgs.shared-mime-info
    ];
    gsettings_schema = pkgs.lib.concatMapStringsSep ":"
      (x: "${x}/share/gsettings-schemas/${x.name}") [
        pkgs.glib
        pkgs.gsettings-desktop-schemas
        pkgs.gtk3
      ];
  in "${base}:${gsettings_schema}";

  preFixup = ''
    gappsWrapperArgs+=(
      --set WEBKIT_DISABLE_COMPOSITING_MODE 1
      --prefix XDG_DATA_DIRS : ${
        pkgs.lib.concatMapStringsSep ":" (x: "${x}/share") [
          pkgs.adwaita-icon-theme
          pkgs.shared-mime-info
        ]
      }
      --prefix XDG_DATA_DIRS : ${
        pkgs.lib.concatMapStringsSep ":"
        (x: "${x}/share/gsettings-schemas/${x.name}") [
          pkgs.glib
          pkgs.gsettings-desktop-schemas
          pkgs.gtk3
        ]
      }
      --prefix GIO_EXTRA_MODULES : ${pkgs.glib-networking}/lib/gio/modules
    )
  '';
}
