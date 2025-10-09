# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Environment

This project uses **Devbox** for environment management. The development environment is configured in `devbox.json`:

```bash
# Enter development environment (sets up Rust toolchain)
devbox shell

# Available scripts
devbox run test     # Run all tests: cargo test
devbox run start    # Run the CLI: cargo run
```

## Common Development Commands

```bash
# Build and run
cargo build                    # Build the project
cargo run                      # Run with no arguments (shows greeting)
cargo run -- [name]           # Run with arguments

# Testing
cargo test                     # Run all tests
cargo test test_greet_with_name # Run specific test

# Code quality
cargo check                    # Quick syntax/type check
cargo clippy                   # Linting
cargo fmt                      # Code formatting
```

## Architecture Overview

**Appa** is a homelab infrastructure management CLI tool designed as an **orchestrator, not implementer**. It coordinates existing tools (Nix, Ansible, Docker, K8s) rather than replacing them.

### Core Architecture Concepts

1. **File-based Source of Truth**: Everything is stored in human-readable YAML files
2. **Plugin Architecture**: Extensible system for different backends (SOPS, BMC, Proxmox, etc.)
3. **Explicit Profile Assignment**: Manual profile assignment prevents accidental changes
4. **Tag-based Organization**: Flat tagging system instead of hierarchies
5. **Mesh-first Networking**: Assumes direct connectivity with Tailscale-inspired ACLs

### Object Model

The system manages five core object types:

- **Systems** (`systems/*.yaml`): Physical/virtual machines with hardware specs, network config, and explicitly assigned profiles
- **Profiles** (`profiles/*.yaml`): Reusable configuration templates that reference modules by backend
- **Modules** (`modules/*/`): Backend-specific configuration files (Nix, Ansible, Docker, shell scripts)
- **Policies** (`policy.yaml`): Single file containing ACL rules and profile enforcement policies
- **Secrets** (`secrets.yaml`): SOPS-encrypted sensitive data referenced via `secrets://` URIs

### CLI Command Structure

The CLI uses a **two-pattern design**:

1. **System-level operations**: `appa <verb> <object> <name>` (CRUD on inventory)
   - `appa add system web-01 --type=server --env=prod`
   - `appa show web-01` (basic properties)

2. **Object modification**: `appa <object> <verb>` (modify existing resources)
   - `appa web-01 show` (detailed system-specific info)
   - `appa web-01 add --profile=nginx-proxy`
   - `appa web-01 deploy --validate`

### Directory Structure

```
homelab/
├── .appa/                      # User preferences & templates
│   ├── config.toml             # LDAP config, defaults
│   ├── plugins.toml            # Plugin configuration
│   └── templates/              # System templates
├── modules/                    # External configurations by tool
│   ├── nix/                    # NixOS configurations
│   ├── ansible/                # Ansible playbooks
│   ├── docker/                 # Docker Compose files
│   └── shell/                  # Shell scripts
├── profiles/                   # Configuration profiles
├── systems/                    # System definitions
├── policy.yaml                 # ACL rules and policies
├── secrets.yaml                # SOPS-encrypted secrets
└── inventory.yaml              # Global index
```

### Plugin System

Plugins extend functionality for specific backends:
- **SOPS Plugin**: Encrypted secrets management (core plugin)
- **BMC Plugin**: Bare metal power management via IPMI/Redfish
- **PXE Plugin**: Network boot and OS installation for bare metal and VMs
- **Proxmox Plugin**: VM lifecycle and hypervisor management
- **Nix/Ansible/Docker Plugins**: Configuration management backends
- **Future Fetch Plugins**: For fetching git remotes, HTTP resources, nixpkgs, etc.

### Key Design Principles

- **Global Naming**: All object names must be globally unique (enables `appa show <name>`)
- **Pure Declarative**: No conditional logic in YAML files - all logic in application code
- **LDAP Integration**: One-way sync from appa to LDAP/FreeIPA for infrastructure groups
- **Zero Trust**: Explicit permissions required (Tailscale-inspired ACL system)
- **Convention over Configuration**: Opinionated defaults with extensibility

## Sample Configuration

The `sample/homelab/` directory contains a complete example homelab setup with:
- Avatar-themed system names (kyoshi, aang, toph, iroh)
- Proxmox VM configurations
- Production/development environment examples
- SOPS-encrypted secrets setup

## Current Development Status

The project is in early development:
- Basic Rust CLI skeleton implemented (`src/main.rs`)
- Comprehensive design documentation (`docs/`)
- Sample configuration examples (`sample/`)
- Plugin architecture designed but not yet implemented

The main implementation work needed:
1. Core CLI command structure with clap
2. YAML file management and validation
3. Plugin system implementation
4. Tag engine and policy validation
5. Git-friendly diffable file formats