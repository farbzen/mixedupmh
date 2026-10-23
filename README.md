<div align="center">

<img src=".assets/logo.png" alt="Mixed Up Monster Hunter" width="128" />

# Mixed Up Monster Hunter

_Ein Zufallsgenerator für Monster Hunter Wilds, der Jagdgruppen Monster und Waffen zulost._

[![Version](https://img.shields.io/badge/version-0.5.1-blue?style=flat-square)](CHANGELOG.md)
[![Rust](https://img.shields.io/badge/Rust-2024-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?style=flat-square&logo=tauri)](https://tauri.app)
[![htmx](https://img.shields.io/badge/htmx-3366CC?style=flat-square&logo=htmx&logoColor=white)](https://htmx.org)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-4-06B6D4?style=flat-square&logo=tailwindcss&logoColor=white)](https://tailwindcss.com)

[Überblick](#überblick) • [Features](#features) • [Schnellstart](#schnellstart) • [Anwendung](#anwendung) • [Entwicklung](#entwicklung) • [Deployment](#deployment) • [Dokumentation](#dokumentation)

</div>

Mixed Up Monster Hunter (kurz _mumh_) ist eine Desktop-App für Gruppen von bis zu
vier Spielern, die ihre Jagden in Monster Hunter Wilds abwechslungsreicher
gestalten wollen: Ein Klick lost das nächste Monster samt Jagdziel und für jeden
Jäger eine Waffe aus.

## Überblick

Die App ist eine [Tauri](https://tauri.app)-Anwendung, die beim Start einen
[axum](https://github.com/tokio-rs/axum)-Server auf einem zufälligen Port an
`127.0.0.1` startet und dessen Oberfläche im Webview-Fenster anzeigt. Die Seiten
werden serverseitig mit [Askama](https://github.com/askama-rs/askama)
gerendert und per [htmx](https://htmx.org) und
[Alpine.js](https://alpinejs.dev) teilweise ausgetauscht; der Zustand (Namen,
Spielerzahl, letzter Wurf, Sprache) liegt in Cookies.

Das Repository besteht aus folgenden Teilen:

- **Anwendung** – Tauri-Fenster und axum-Router, gebaut mit [Rust](https://www.rust-lang.org). Der Code liegt in `src/`.
- **Templates** – Seiten, Layouts und htmx-Komponenten. Sie liegen in `templates/`.
- **Spieldaten** – Monster, Waffen und Übersetzungen als JSON. Sie liegen in `json/`.
- **Statische Dateien** – Bilder, JavaScript und das generierte CSS (Quelle: `tailwind.css`). Sie liegen in `static/`.

## Features

- **Monster-Roulette**: lost eines von 39 Monstern aus, inklusive Stufe (normal, gehärtet, erzgehärtet), sofern das Monster sie kennt.
- **Jagdziel**: entscheidet bei fangbaren Monstern per Münzwurf zwischen Fangen und Töten.
- **Waffen-Roulette**: weist jedem von 1–4 Jägern eine der 14 Waffenarten zu.
- **Eigene Jägernamen**: Namen und Spielerzahl bleiben zwischen den Sitzungen erhalten.
- **Zweisprachig**: Oberfläche auf Englisch und Deutsch, umschaltbar im Footer.
- **Plattformübergreifend**: Builds für Linux (AppImage, deb, rpm), macOS (Apple Silicon und Intel) und Windows.

## Schnellstart

### Voraussetzungen

- [devenv](https://devenv.sh) (stellt Rust, Tauri-CLI, Tailwind CSS und die GTK/WebKit-Bibliotheken bereit)
- optional [direnv](https://direnv.net) zum automatischen Laden der Umgebung (`.envrc`)

### Entwicklungsumgebung

```sh
git clone git@github.com:farbzen/mixedupmh.git
cd mixedupmh
devenv shell
```

> [!TIP]
> Fertige Installationspakete für alle Plattformen gibt es unter
> [Releases](https://github.com/farbzen/mixedupmh/releases) – zum Spielen ist
> keine Entwicklungsumgebung nötig.

## Anwendung

App im Entwicklungsmodus starten:

```sh
cargo tauri dev
```

CSS bei Änderungen an Templates oder `tailwind.css` neu erzeugen (Watch-Modus,
`tailwindmini` erzeugt minifiziertes CSS):

```sh
tailwind
```

> [!NOTE]
> Im Entwicklungsmodus liest die App `static/` und `json/` direkt aus dem
> Projektverzeichnis; erst ein `cargo tauri build` bündelt sie als Ressourcen.
> Änderungen an den JSON-Dateien greifen deshalb ohne Neubau.

Installationspakete lokal bauen:

```sh
cargo tauri build
```

## Entwicklung

### Code-Qualität

Formatierung nach `rustfmt.toml` (Zeilenbreite 80) und Lint mit Clippy:

```sh
cargo fmt
cargo clippy
```

## Deployment

Releases baut der GitHub-Workflow `.github/workflows/release.yml` mit
[tauri-action](https://github.com/tauri-apps/tauri-action) für Linux, macOS und
Windows und hängt die Pakete an ein GitHub-Release. Er startet beim Push eines
Tags `v*` oder manuell:

```sh
git tag v0.5.1
git push origin v0.5.1
```

> [!IMPORTANT]
> Die Version steht in `version`, `Cargo.toml` und `tauri.conf.json` und muss
> in allen drei Dateien angehoben und in [CHANGELOG.md](CHANGELOG.md)
> eingetragen werden; der Release-Name wird aus der Tauri-Version gebildet.

## Dokumentation

- [Spieldaten pflegen](docs/spieldaten.md) – Monster, Waffen und Sprachen ergänzen
- [Changelog](CHANGELOG.md)
