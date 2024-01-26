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
  ];

  # https://devenv.sh/scripts/
  # scripts.hello.exec = "echo hello from $GREET";

  # enterShell = ''
  #   hello
  #   git --version
  # '';

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

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
