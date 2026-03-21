{pkgs, ...}: {
  packages = with pkgs; [
    tailwindcss_4
    pkg-config
    prettierd
  ];

  languages.rust.enable = true;

  scripts = {
    tailwind.exec = ''
      tailwindcss -i tools/tailwind/tailwind.css -o res/static/css/main.css -w
    '';

    tailwindmini.exec = ''
      tailwindcss -i tools/tailwind/tailwind.css -o res/static/css/main.css -m -w
    '';
  };
}
