{ pkgs, ... }:

{
  # https://devenv.sh/basics/

  # https://devenv.sh/integrations/codespaces-devcontainer/
  devcontainer.enable = true;

  # https://devenv.sh/packages/
  packages = [
    pkgs.git
    pkgs.bun
    pkgs.cargo-watch
    pkgs.tailwindcss-language-server
    pkgs.rustywind
  ];

  # https://devenv.sh/scripts/
  scripts.publish.exec = "cargo run --release";

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/pre-commit-hooks/
  pre-commit.hooks = {
    actionlint.enable = true;
    cargo-check.enable = true;
    clippy.enable = true;
    rustfmt.enable = true;
    rustywind = {
      enable = true;
      entry = "rustywind";
      files = "\.rs$";
    };
  };

  # https://devenv.sh/processes/
  processes.watch.exec = "cargo watch --exec run";

  # https://devenv.sh/services/
  services.caddy = {
    enable = true;
    config = ''
      localhost:8080

      root * ./_site
      file_server
    '';
  };

  # See full reference at https://devenv.sh/reference/options/
}
