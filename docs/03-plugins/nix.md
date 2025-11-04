# Nix Plugin

The Nix plugin provides NixOS configuration management with flake support and declarative system configuration.

## Overview

- **Purpose**: NixOS system configuration and package management
- **Features**: Declarative configs, reproducible builds, rollbacks
- **Integration**: Flakes, channels, remote builds
- **Status**: Optional plugin for NixOS systems

## Configuration

### Plugin Settings (.appa/plugins.toml)
```toml
[plugins.nix]
enabled = true
default_channel = "nixos-23.11"
flake_support = true
remote_build = false
```

### System Configuration
```yaml
# In systems/kyoshi.yaml
backend: nix
backend_config:
  channel: "nixos-23.11"
  flake_ref: "github:myorg/homelab-nix"
```

## Features

### Declarative Configuration
- **Pure Configs**: No imperative state changes
- **Reproducible**: Same config produces identical systems
- **Rollbacks**: Atomic rollback to previous configurations
- **Atomic Updates**: All-or-nothing system updates

### Flake Support
- **Hermetic Builds**: Pinned dependencies for reproducibility
- **Composition**: Combine multiple flakes and inputs
- **Development Shells**: Consistent development environments
- **Cross-Platform**: Build for different architectures

### Package Management
- **Declarative Packages**: Packages defined in configuration
- **Multiple Channels**: Stable and unstable package sets
- **Overlays**: Custom package modifications
- **User Packages**: Per-user package management

## Module Structure

### Module Definition
```nix
# modules/nix/web-server.nix
{ config, lib, pkgs, ... }:
with lib;
{
  options = {
    webServer = {
      domain = mkOption {
        type = types.str;
        default = "localhost";
        description = "Primary domain for the web server";
      };

      enableSSL = mkOption {
        type = types.bool;
        default = false;
        description = "Enable SSL/TLS termination";
      };
    };
  };

  config = {
    services.nginx = {
      enable = true;
      virtualHosts."${config.webServer.domain}" = {
        enableACME = config.webServer.enableSSL;
        forceSSL = config.webServer.enableSSL;
        locations."/" = {
          root = "/var/www";
          index = "index.html";
        };
      };
    };

    networking.firewall.allowedTCPPorts = [ 80 ] ++
      optionals config.webServer.enableSSL [ 443 ];
  };
}
```

### Module Registration
```yaml
# In profiles/nginx-proxy.yaml
configurations:
  - backend: nix
    module: "web-server-nix"
    variables:
      domain: "kyoshi.avatar"
      enableSSL: true
```

## CLI Commands

### System Management
```bash
# Configuration management
appa kyoshi nix build              # Build configuration
appa kyoshi nix switch             # Apply configuration
appa kyoshi nix test               # Test configuration
appa kyoshi nix rollback           # Rollback to previous config

# Channel management
appa kyoshi nix update-channels    # Update Nix channels
appa kyoshi nix show-generations   # Show system generations
```

### Package Management
```bash
# Package operations
appa kyoshi nix install htop       # Install package temporarily
appa kyoshi nix search nginx       # Search for packages
appa kyoshi nix show-packages      # List installed packages
```

### Flake Operations
```bash
# Flake management
appa kyoshi nix flake-update       # Update flake inputs
appa kyoshi nix flake-show         # Show flake outputs
appa kyoshi nix flake-check        # Check flake validity
```

## Integration Points

### With Proxmox Plugin
1. **VM Creation**: Proxmox creates VM from NixOS template
2. **Initial Boot**: Minimal NixOS system boots
3. **Nix Plugin**: Applies full system configuration
4. **Updates**: Nix manages all system state

### With Secrets Plugin
```nix
# Reference secrets in Nix config
config.services.myservice.passwordFile = config.age.secrets.myservice-password.path;
```

## Configuration Patterns

### Service Configuration
```nix
# Enable and configure services
services = {
  nginx.enable = true;
  postgresql.enable = true;
  openssh = {
    enable = true;
    settings.PasswordAuthentication = false;
  };
};
```

### User Management
```nix
# Define users declaratively
users.users.alice = {
  isNormalUser = true;
  extraGroups = [ "wheel" "docker" ];
  openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3... alice@laptop"
  ];
};
```

### Network Configuration
```nix
# Network settings
networking = {
  hostName = "kyoshi";
  domain = "avatar";
  firewall = {
    enable = true;
    allowedTCPPorts = [ 22 80 443 ];
  };
};
```

## Advanced Features

### Custom Overlays
```nix
# Custom package overlay
nixpkgs.overlays = [
  (final: prev: {
    myCustomPackage = prev.callPackage ./packages/my-package {};
  })
];
```

### Secrets Management with agenix
```nix
# Integration with age secrets
age.secrets.gitlab-password = {
  file = ./secrets/gitlab-password.age;
  owner = "gitlab";
  group = "gitlab";
};
```

### Development Environments
```nix
# Development shell
devShells.default = pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rust-analyzer
  ];
};
```

## Best Practices

### Configuration Organization
- **Modular Configs**: Break configuration into logical modules
- **Shared Code**: Common configurations in shared modules
- **Environment-Specific**: Separate configs for prod/dev/staging

### Version Management
- **Pin Channels**: Use specific channel versions for stability
- **Flake Locks**: Lock flake inputs for reproducibility
- **Testing**: Test configurations before applying to production

### Security
- **Minimal Installs**: Only install necessary packages
- **Firewall Rules**: Configure firewall declaratively
- **User Permissions**: Use least privilege principle

## Examples

See the [sample homelab modules](../../sample/homelab/modules/nix/) for complete Nix configuration examples.