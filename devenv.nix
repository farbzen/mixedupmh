{pkgs, ...}: {
  packages = with pkgs; [
    tailwindcss_4
    pkg-config
    prettierd
  ];

  languages.rust.enable = true;

  scripts = {
    tailwind.exec = ''
      tailwindcss -i tailwind.css -o static/css/main.css -w
    '';

    tailwindmini.exec = ''
      tailwindcss -i tailwind.css -o static/css/main.css -m -w
    '';
  };
}
