{pkgs, ...}: {
  packages = [pkgs.tailwindcss];

  languages.rust.enable = true;

  scripts = {
    tailwind.exec = ''
      tailwindcss -c tools/tailwind/tailwind.config.ts -i tools/tailwind/tailwind.css -o res/static/css/main.css -w
    '';

    tailwindmini.exec = ''
      tailwindcss -c tools/tailwind/tailwind.config.ts -i tools/tailwind/tailwind.css -o res/static/css/main.css -m -w
    '';
  };
}
