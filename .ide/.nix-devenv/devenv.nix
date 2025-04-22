{pkgs, ...}: {
  packages = [
    pkgs.tailwindcss
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
