{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    home-manager,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        zsh-autosuggestions-src = pkgs.fetchFromGitHub {
          owner = "zsh-users";
          repo = "zsh-autosuggestions";
          rev = "v0.7.0";
          sha256 = "KLUYpUu4DHRumQZ3w59m9aTW6TBKMCXl2UcKi4uMd7w=";
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust tools
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
            go

            # Shell utilities
            zsh
            oh-my-zsh
            zsh-syntax-highlighting
            fzf
            direnv

            # Build essentials
            pkg-config
            openssl
            openssl.dev

            # Development tools
            git
            ripgrep
            fd
          ];

          shellHook = ''
            # Use zsh as the default shell
            export SHELL=${pkgs.zsh}/bin/zsh
            exec ${pkgs.zsh}/bin/zsh

            # Git configuration
            git config --global user.name "alswang18"
            git config --global user.email "alec.sy.wang@gmail.com"

            # ZSH configuration
            export ZDOTDIR="$HOME/.config/zsh"
            export ZSH=${pkgs.oh-my-zsh}/share/oh-my-zsh
            export ZSH_CUSTOM="$ZDOTDIR/custom"

            # Ensure directories exist
            mkdir -p "$ZDOTDIR/plugins"
            mkdir -p "$ZSH_CUSTOM/plugins"

            # Set up custom autosuggestions from specified version
            ln -sf ${zsh-autosuggestions-src} \
              "$ZSH_CUSTOM/plugins/zsh-autosuggestions"

            # Link syntax highlighting
            ln -sf ${pkgs.zsh-syntax-highlighting}/share/zsh-syntax-highlighting \
              "$ZSH_CUSTOM/plugins/zsh-syntax-highlighting"

            # Enable plugins
            export ZSH_PLUGINS="git z zsh-autosuggestions zsh-syntax-highlighting"

            if [ -f "$ZDOTDIR/.zshrc" ]; then
              source "$ZDOTDIR/.zshrc"
            else
              source $ZSH/oh-my-zsh.sh
            fi

            # Set up Rust environment variables
            export RUST_SRC_PATH="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}"

            # Initialize direnv if it exists
            if command -v direnv >/dev/null 2>&1; then
              eval "$(direnv hook zsh)"
            fi
          '';

          # Preserve SSL certificates path
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        };
      }
    );

  nixConfig = {
    # Allow dirty git tree during development
    allow-dirty = true;
  };
}
