{ pkgs ? import <nixos-unstable> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.git
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.clippy
    pkgs.tmux
pkgs.pkg-config  # Add pkg-config
    pkgs.fontconfig  # Add fontconfig
  ];
}

