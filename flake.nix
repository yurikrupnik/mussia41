# https://www.youtube.com/watch?v=LE5JR4JcvMg
{
    description = "my minimal flake";
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/master";
        home-manager.url = "github:nix-community/home-manager/master";
        home-manager.inputs.nixpkgs.follows = "nixpkgs";

        darwin.url = "github:lnl7/nix-darwin";
        darwin.inputs.nixpkgs.follows = "nixpkgs";
    };
    output = inputs: {
        darwinConfigurations.Demo-shit = inputs.darwin.lib.darwinSystem {
          system = "aarch64-darwin";
          pkgs = import inputs.nixpkgs {inherit system;};
        };
        modules = [
            ({pkgs, ...}: {
                programs.zsh.enable = true;
                environment.shells = [pkgs.bash pkgs.zsh];
                environment.loginShell = pkgs.zsh;
                nix.extraOptions = ''
                    experimental-features = nix-command flakes
                '';
                systemPackages = [pkgs.coreutils];
                system.keyboard.enableKeyMapping = true;
                fonts.fontDir.enable = true; # Danger!
                services.nix-daemon.enable = true;
                system.defaults.dock.autohide = true;
                system.defaults.NSGlobalDomain.InitialKeyRepeat = 14;
                system.defaults.NSGlobalDomain.KeyRepeat = 1;
                system.defaults.NSGlobalDomain.AppleShowAllExtensions = true;
                system.defaults.finder.AppleShowAllExtensions = true;
                system.defaults.finder.FXShowPosixPathInTiltle = true;
                homebrew = {
                  enable = true;
                  caskArgs.no_quarantine = true;
                  global.brewfile = true;
                  masArpps = {};
                  casks = ["raycast" "amethyst"];
#                  taps = ["fujiapple852/trippy"];
                  brews = ["trippy"];
                };
            })
        ];
    };
}