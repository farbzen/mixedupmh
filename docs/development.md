# Development

All commands run inside the devenv shell (`devenv shell`, or automatically via
direnv).

## Setup

[devenv](https://devenv.sh) provides Rust, the Tauri CLI, Tailwind CSS and the
GTK/WebKit libraries; [direnv](https://direnv.net) optionally loads the
environment automatically (`.envrc`).

```sh
git clone git@github.com:farbzen/mixedupmh.git
cd mixedupmh
devenv shell
```

## Project layout

| Path            | Content                                                  |
| --------------- | -------------------------------------------------------- |
| `src/`          | Tauri window, axum router, handlers, cookie handling     |
| `templates/`    | Askama pages, layouts and htmx components                |
| `json/`         | monsters, weapons and translations                       |
| `static/`       | images, JavaScript and the generated CSS                 |
| `tailwind.css`  | Tailwind CSS source for `static/css/main.css`            |
| `icons/`        | app icons for the installers                             |
| `nix/`          | Nix derivation used by the flake                         |

## Running

```sh
cargo tauri dev
```

In development mode the app reads `static/` and `json/` directly from the
project directory; only `cargo tauri build` bundles them as resources. Changes
to the JSON files therefore take effect without a rebuild.

## CSS

The devenv scripts regenerate `static/css/main.css` in watch mode whenever
templates or `tailwind.css` change:

```sh
tailwind      # readable output
tailwindmini  # minified output
```

## Code quality

Formatting according to `rustfmt.toml` (line width 80) and linting with Clippy:

```sh
cargo fmt
cargo clippy
```

## Building installers locally

```sh
cargo tauri build
```

## Releases

The GitHub workflow `.github/workflows/release.yml` builds releases for Linux,
macOS and Windows with
[tauri-action](https://github.com/tauri-apps/tauri-action) and attaches the
packages to a GitHub release. It runs when a `v*` tag is pushed, or manually:

```sh
git tag v0.8.0
git push origin v0.8.0
```

> [!IMPORTANT]
> The version is stored in `version`, `Cargo.toml` and `tauri.conf.json`; it
> must be bumped in all three files and recorded in
> [CHANGELOG.md](../CHANGELOG.md). The release name is derived from the Tauri
> version.
