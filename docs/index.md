# Appa Documentation

Welcome to the Appa documentation. Appa is a CLI tool for homelab infrastructure orchestration and inventory management that acts as an orchestrator, coordinating existing tools rather than replacing them.

## Getting Started

- **[CLI Commands](cli.md)** - Complete CLI reference and subcommand documentation
- **[Design Document](design.md)** - Complete overview of Appa's architecture, philosophy, and implementation
- **[Design Decisions](dd.md)** - Key design decisions with rationale and alternatives considered

## Core Concepts

### Object Types

- **[Objects Overview](objects.md)** - Common properties and patterns shared by all object types

Appa manages five core object types in your homelab:

- **[Systems](system.md)** - Physical and virtual machines with hardware specs and network configuration
- **[Profiles](profile.md)** - Reusable configuration templates that reference modules by backend
- **[Modules](module.md)** - Backend-specific configuration files (Nix, Ansible, Docker, shell scripts)
- **[Policies](policy.md)** - Access control rules and profile enforcement policies
- **[Secrets](secret.md)** - SOPS-encrypted sensitive data referenced via `secrets://` URIs

### Plugin Architecture

Appa uses plugins to extend functionality for specific backends and services:

#### Core Infrastructure Plugins
- **[SOPS Plugin](plugins/sops.md)** - Encrypted secrets management using Mozilla SOPS
- **[BMC Plugin](plugins/bmc.md)** - Bare metal power management via IPMI/Redfish
- **[PXE Plugin](plugins/pxe.md)** - Network boot and OS installation for bare metal and VMs
- **[LDAP Plugin](plugins/ldap.md)** - LDAP/FreeIPA integration for infrastructure groups

#### Virtualization Plugins
- **[Proxmox Plugin](plugins/proxmox.md)** - Hypervisor and VM lifecycle management

#### Configuration Management Plugins
- **[Nix Plugin](plugins/nix.md)** - NixOS configuration management
- **[Ansible Plugin](plugins/ansible.md)** - Playbook-based configuration
- **[Docker Plugin](plugins/docker.md)** - Container orchestration

## Key Features

### CLI Design
- **Consistent command structure**: `appa <object> <verb> [name]` pattern with object names as positional arguments
- **Global naming**: All objects have globally unique names
- **Explicit profile assignment**: Manual profile assignment prevents accidental configuration changes

### Infrastructure as Code
- **File-based source of truth**: Human-readable YAML files
- **VCS-friendly**: Diffable configuration files designed for version control
- **Pure declarative**: No conditional logic in configuration files

### Integration Strategy
- **Orchestrator, not implementer**: Coordinates existing tools (Nix, Ansible, Docker, K8s)
- **Plugin-based extensibility**: Modular architecture for different backends
- **Bootstrap workflow**: BMC + PXE plugins provision systems, then hand off to configuration management

### Security and Access Control
- **Zero trust by default**: Explicit permissions required (Tailscale-inspired ACL system)
- **LDAP integration**: One-way sync to LDAP/FreeIPA for infrastructure groups
- **Encrypted secrets**: SOPS integration for secure credential management

## Documentation Structure

```
docs/
├── index.md                    # This overview (you are here)
├── cli.md                      # CLI commands and subcommand reference
├── design.md                   # Complete design document
├── dd.md                       # Design decisions and rationale
├── objects.md                  # Common object properties and patterns
├── system.md                   # Physical/virtual machine definitions
├── profile.md                  # Configuration templates
├── module.md                   # Backend-specific configurations
├── policy.md                   # Access control and enforcement
├── secret.md                   # Encrypted configuration values
├── modules/                    # Module-specific documentation
│   └── module-interface.md     # Module interface and plugin handler system
├── subcommands/                # CLI subcommand documentation
│   └── maintainers.md          # Maintainer management commands
└── plugins/                    # Plugin-specific documentation
    ├── sops.md                # Secrets management
    ├── bmc.md                 # Bare metal provisioning
    ├── pxe.md                 # Network boot and installation
    ├── ldap.md                # LDAP/FreeIPA integration
    ├── proxmox.md             # Hypervisor management
    ├── nix.md                 # NixOS configuration
    ├── ansible.md             # Ansible playbooks
    └── docker.md              # Container orchestration
```

## Example Configuration

See the [sample homelab](../sample/homelab/) directory for complete examples including:
- Avatar-themed system configurations (kyoshi, aang, toph, iroh)
- Production and development environment setups
- Plugin configurations and SOPS-encrypted secrets
- Profile definitions and policy examples

## Development Status

Appa is currently in early development:
- ✅ Comprehensive design and architecture documentation
- ✅ Plugin architecture design
- ✅ Sample configuration examples
- 🚧 Core CLI implementation (in progress)
- 🚧 Plugin system implementation
- 🚧 YAML file management and validation

See the main [repository](../) for the current Rust implementation and [CLAUDE.md](../CLAUDE.md) for development guidance.