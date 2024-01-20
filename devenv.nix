{ pkgs, ... }:

{
  devcontainer.enable = true;

  # https://devenv.sh/basics/
  env.MINISERVE_PATH = "_site";
  env.MINISERVE_INDEX = "index.html";

  # https://devenv.sh/packages/
  packages = [
    pkgs.git
    pkgs.bun
    pkgs.cargo-watch
    pkgs.miniserve
  ];

  # https://devenv.sh/scripts/
  # scripts.hello.exec = "echo hello from $GREET";

  # enterShell = ''
  #   hello
  #   git --version
  # '';

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  processes.watch.exec = "cargo watch --exec run";
  processes.serve.exec = "miniserve";

  # See full reference at https://devenv.sh/reference/options/
}
