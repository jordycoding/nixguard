# Nixguard
A way to easily manage vpn setups under NixOS by generating a .toml file to import in your nixos config.

## How to use
Include this package as an input in your flake.nix file, ```nixguard.url = "github:jordycoding/nixguard";```. And include the package somewhere in your config ```inputs.nixguard.packages.x86_64-linux.default```.

Now you can run ```nixguard add``` in your config folder. And follow the steps. Save the secret key somewhere safe(preferably somewhere normal users can't access it, e.g. agenix). Now you can import config.toml in your nixos config.
E.g:
```nix
{ config, pkgs, lib, ... }:
with pkgs;
let
  vpnconfig = builtins.fromTOML (builtins.readFile ../../config.toml);
in
{
  networking.firewall.allowedUDPPorts = [ 51820 ];
  systemd.network = {
    netdevs = vpnconfig.netdevs;
    networks = vpnconfig.networks;
  };
}
```
Don't forget to enable systemd.networkd:
```nix
  networking.useNetworkd = true;
  systemd.network.enable = true;
```
