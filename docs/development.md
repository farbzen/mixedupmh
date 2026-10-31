# Development

All commands run inside the devenv shell (`devenv shell`, or automatically via
direnv).

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
