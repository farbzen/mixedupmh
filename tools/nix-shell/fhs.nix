{pkgs ? import <nixpkgs> {}}:
pkgs.buildFHSEnv {
  name = "mumh-fhs";
}
