{pkgs, ...}: {
  packages = with pkgs; [
    tailwindcss_4
    pkg-config
    prettierd

    cargo-tauri
    webkitgtk_4_1
    gtk3
    glib
    cairo
    gdk-pixbuf
    gobject-introspection
    at-spi2-atk
    atkmm
    pango
    harfbuzz
    librsvg
    libsoup_3
    dbus
    openssl
    wrapGAppsHook4
  ];

  languages.rust.enable = true;

  enterShell = ''
    export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH:$XDG_DATA_DIRS"
  '';

  scripts = {
    tailwind.exec = ''
      tailwindcss -i tailwind.css -o static/css/main.css -w
    '';

    tailwindmini.exec = ''
      tailwindcss -i tailwind.css -o static/css/main.css -m -w
    '';
  };
}
