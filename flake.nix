{
  description = "Mixed Up Monster Hunter – a randomizer for Monster Hunter Wilds";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forEachSystem =
        buildOutputs:
        nixpkgs.lib.genAttrs supportedSystems (system: buildOutputs nixpkgs.legacyPackages.${system});
    in
    {
      packages = forEachSystem (pkgs: rec {
        mixedupmh = pkgs.callPackage ./nix/package.nix { };
        default = mixedupmh;
      });

      overlays.default = final: _prev: {
        mixedupmh = final.callPackage ./nix/package.nix { };
      };
    };
}
