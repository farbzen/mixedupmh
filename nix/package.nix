{
  lib,
  rustPlatform,
  cargo-tauri,
  pkg-config,
  wrapGAppsHook4,
  glib-networking,
  openssl,
  webkitgtk_4_1,
}:

let
  projectRoot = ../.;
in
rustPlatform.buildRustPackage {
  pname = "mixedupmh";
  version = lib.fileContents (projectRoot + "/version");

  src = lib.fileset.toSource {
    root = projectRoot;
    fileset = lib.fileset.unions [
      (projectRoot + "/Cargo.toml")
      (projectRoot + "/Cargo.lock")
      (projectRoot + "/build.rs")
      (projectRoot + "/askama.toml")
      (projectRoot + "/tauri.conf.json")
      (projectRoot + "/icons")
      (projectRoot + "/json")
      (projectRoot + "/src")
      (projectRoot + "/static")
      (projectRoot + "/templates")
    ];
  };

  cargoLock.lockFile = projectRoot + "/Cargo.lock";

  nativeBuildInputs = [
    cargo-tauri.hook
    pkg-config
    wrapGAppsHook4
  ];

  buildInputs = [
    glib-networking
    openssl
    webkitgtk_4_1
  ];

  meta = {
    description = "Randomizer for Monster Hunter Wilds that rolls monsters and weapons for your hunting party";
    homepage = "https://github.com/farbzen/mixedupmh";
    platforms = lib.platforms.linux;
    mainProgram = "mumh";
  };
}
