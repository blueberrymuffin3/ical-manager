{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  # nativeBuildInputs is usually what you want -- tools you need to run
  nativeBuildInputs = [
    pkgs.rustup
    pkgs.cargo-watch
    pkgs.cargo-expand
    pkgs.sqlx-cli
    pkgs.sqlite-interactive
    
    pkgs.clang
    pkgs.mold
  ];
}
