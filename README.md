<div align="center">

<img src=".assets/logo.png" alt="Mixed Up Monster Hunter" width="128" />

# Mixed Up Monster Hunter

_A randomizer for Monster Hunter Wilds that rolls monsters and weapons for your hunting party._

[![Version](https://img.shields.io/badge/version-0.8.0-blue?style=for-the-badge&labelColor=081216)](CHANGELOG.md)
[![Rust](https://img.shields.io/badge/Rust-2024-orange?style=for-the-badge&labelColor=081216&logo=rust)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?style=for-the-badge&labelColor=081216&logo=tauri)](https://tauri.app)
[![htmx](https://img.shields.io/badge/htmx-3366CC?style=for-the-badge&labelColor=081216&logo=htmx&logoColor=white)](https://htmx.org)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-4-06B6D4?style=for-the-badge&labelColor=081216&logo=tailwindcss&logoColor=white)](https://tailwindcss.com)
[![License: MIT](https://img.shields.io/badge/license-MIT-B0BFC9?style=for-the-badge&labelColor=081216)](LICENSE)
[![Ko-Fi donate](https://img.shields.io/badge/donate-kofi?style=for-the-badge&labelColor=081216&color=ee5556&logo=ko-fi&logoColor=white)](https://ko-fi.com/farbzen)

[Overview](#overview) • [Features](#features) •
[Getting Started](#getting-started) • [Documentation](#documentation)

</div>

Mixed Up Monster Hunter (_mumh_ for short) is a desktop app for groups of up to
four players who want more variety in their Monster Hunter Wilds hunts: a single
click rolls the next monster along with the hunt objective and a weapon for
every hunter.

## Overview

The app is a [Tauri](https://tauri.app) application that starts an
[axum](https://github.com/tokio-rs/axum) server on a random port on `127.0.0.1`
at launch and displays its UI in the webview window. Pages are rendered
server-side with [Askama](https://github.com/askama-rs/askama) and partially
swapped via [htmx](https://htmx.org) and [Alpine.js](https://alpinejs.dev); the
state (names, player count, last roll, language) is kept in cookies.

The repository consists of the following parts:

- **mumh** – Tauri window and axum router, built with
  [Rust](https://www.rust-lang.org). The code lives in `src/`.
- **Templates** – pages, layouts and htmx components, built with
  [Askama](https://github.com/askama-rs/askama). They live in `templates/`.
- **Game data** – monsters, weapons and translations as JSON. They live in
  `json/`.
- **Static files** – images, JavaScript and the CSS generated with
  [Tailwind CSS](https://tailwindcss.com) from `tailwind.css`. They live in
  `static/`.

## Features

- **Monster roulette**: rolls one of 39 monsters, including its tier (normal,
  tempered, arch-tempered) where the monster has one.
- **Hunt objective**: flips a coin between capture and slay for capturable
  monsters.
- **Weapon roulette**: assigns one of the 14 weapon types to each of 1–4
  hunters.
- **Custom hunter names**: names and player count persist between sessions.
- **Bilingual**: UI in English and German, switchable in the footer.
- **Cross-platform**: installers for Linux, macOS and Windows, plus a Nix flake
  for NixOS.

## Getting Started

### Prerequisites

- Linux (x86_64), macOS (Apple Silicon or Intel) or Windows
- optional: [Nix](https://nixos.org) with flakes enabled

### Installation

Download the package for your platform from the latest
[release](https://github.com/farbzen/mixedupmh/releases):

| Platform | Packages                  |
| -------- | ------------------------- |
| Linux    | AppImage, `.deb`, `.rpm`  |
| macOS    | `.dmg` (aarch64, x64)     |
| Windows  | `.msi`, setup `.exe`      |

On Nix, run the app straight from the flake without installing it:

```sh
nix run github:farbzen/mixedupmh
```

> [!TIP]
> To install the app permanently on NixOS or with Home Manager, add the flake's
> overlay as described in [Nix and NixOS](docs/nix.md).

To build and run the app from source, follow
[Development](docs/development.md).

## Documentation

- [Development](docs/development.md) – setup, project layout, CSS, code
  quality, local builds, releases
- [Maintaining game data](docs/game-data.md) – adding monsters, weapons and
  languages
- [Nix and NixOS](docs/nix.md) – flake outputs, installation on NixOS, local
  builds
- [Changelog](CHANGELOG.md)

## License

See [LICENSE](LICENSE).
