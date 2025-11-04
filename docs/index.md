# Appa Documentation

Welcome to the Appa documentation. Appa is a CLI tool for homelab infrastructure orchestration and inventory management that acts as an orchestrator, coordinating existing tools rather than replacing them.

## Quick Navigation

### 🚀 [Getting Started](00-getting-started/)
New to Appa? Start here for installation, setup, and your first system configuration.

### 📖 [User Guide](01-user-guide/)
Key concepts, workflows, and practical examples for daily operations.


### 🏗️ [Architecture](02-architecture/)
Deep dive into Appa's design, object model, and plugin system.

### 🔌 [Plugins](03-plugins/)
Plugin-specific documentation for extending Appa with different backends and services.

### 📚 [Reference](04-reference/)
Complete CLI command reference, subcommands, examples, and troubleshooting.

**Quick CLI Access:**
- [CLI Reference](04-reference/cli-reference.md) - Complete command documentation
- [System Commands](04-reference/cli-system.md) - Infrastructure management
- [Plugin Commands](04-reference/cli-plugin.md) - Extension management

### 📝 [Contributing](05-contributing/)
Guidelines for contributing to documentation and maintaining consistency.

---

## What is Appa?

Appa is a homelab infrastructure management CLI that:

- **Orchestrates existing tools** (Nix, Ansible, Docker, K8s) rather than replacing them
- **Provides a file-based source of truth** for systems, profiles, and policies
- **Uses a plugin architecture** for extensibility with different backends
- **Integrates with LDAP/FreeIPA** for infrastructure group management
- **Assumes mesh networking** with Tailscale-inspired ACL system

## Core Object Types

- **[Systems](01-user-guide/managing-systems.md)** - Physical and virtual machines
- **[Profiles](01-user-guide/profile.md)** - Reusable configuration templates
- **[Modules](01-user-guide/modules.md)** - Backend-agnostic service configuration
- **[Plugins](04-reference/cli-plugin.md)** - Runtime-loadable extensions that add new subcommands
- **[Policies](01-user-guide/policy.md)** - Access control rules
- **[Secrets](01-user-guide/secret.md)** - Encrypted sensitive data

## Development Status

Appa is currently in early development:
- ✅ Comprehensive design and architecture documentation
- ✅ Plugin architecture design
- ✅ Sample configuration examples
- 🚧 Core CLI implementation (in progress)
- 🚧 Plugin system implementation
- 🚧 YAML file management and validation

See the main [repository](../) for the current Rust implementation and [CLAUDE.md](../CLAUDE.md) for development guidance.