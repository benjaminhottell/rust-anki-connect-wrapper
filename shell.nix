{ pkgs ? import <nixpkgs> {} }:
let

  shell = pkgs.mkShell {

    buildInputs = [

      # Dependencies
      pkgs.openssl
      pkgs.pkg-config

      # Dev tools
      pkgs.cargo
      pkgs.clippy

    ];

  };

in
  shell
