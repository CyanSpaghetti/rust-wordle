{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    buildInputs = [
        pkgs.rustup
        pkgs.cargo
        pkgs.pkg-config
        pkgs.libjpeg
        pkgs.xorg.libX11
        pkgs.xorg.libxcb
        pkgs.mesa
    ];
    shellHook = ''
    export PATH=$HOME/.cargo/bin:$PATH
        '';
}

