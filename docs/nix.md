# Nix and NixOS

The repository is a Nix flake. The derivation lives in `nix/package.nix` and is
built with `rustPlatform.buildRustPackage` and `cargo-tauri.hook`, which
installs the binary to `bin/mumh` and the resources (`static/`, `json/`) to
`lib/Mixed Up Monster Hunter/`, where Tauri looks them up at runtime.

Supported systems: `x86_64-linux`, `aarch64-linux`.

## Flake outputs

| Output                         | Content                                  |
| ------------------------------ | ---------------------------------------- |
| `packages.<system>.mixedupmh`  | the application                          |
| `packages.<system>.default`    | alias for `mixedupmh`                    |
| `overlays.default`             | adds `mixedupmh` to `pkgs`               |

## Run without installing

```sh
nix run github:farbzen/mixedupmh
```

## Install on NixOS

Add the flake as an input and apply the overlay in your system flake:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    mixedupmh = {
      url = "github:farbzen/mixedupmh";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, mixedupmh, ... }:
    {
      nixosConfigurations.<hostname> = nixpkgs.lib.nixosSystem {
        modules = [
          (
            { pkgs, ... }:
            {
              nixpkgs.overlays = [ mixedupmh.overlays.default ];
              environment.systemPackages = [ pkgs.mixedupmh ];
            }
          )
          ./configuration.nix
        ];
      };
    };
}
```

For Home Manager, use `home.packages` instead of
`environment.systemPackages`.

To pin a release, use a tag as the URL, e.g. `github:farbzen/mixedupmh/v0.5.1`.

## Build locally

```sh
nix build
./result/bin/mumh
```

## Maintenance

- The package version is read from `/version`.
- Dependencies are taken from `Cargo.lock`; no hash has to be updated after
  dependency changes.
- New files or directories required at build time must be added to the
  `fileset` in `nix/package.nix`.
