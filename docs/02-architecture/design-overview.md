# Appa Homelab Infrastructure Management Tool - Design Document

## Overview

Appa is a CLI tool for homelab infrastructure orchestration and inventory management. It provides a file-based source of truth for systems, networks, and access control, then delegates actual configuration management to existing tools like Nix, Ansible, Docker, and Kubernetes. It integrates with LDAP/FreeIPA for user management, takes inspiration from Tailscale's ACL system, and assumes mesh-network connectivity with opinionated defaults.

## Design Philosophy

- **Orchestrator, not implementer**: Coordinates existing tools rather than replacing them
- **Infrastructure, not people**: Manages systems/networks/policies, not users
- **LDAP integration**: One-way sync to LDAP/FreeIPA - appa is source of truth for infrastructure
- **Delegation over reinvention**: Use Nix, Ansible, Docker, K8s for actual configuration
- **Mesh-first networking**: Assume direct connectivity between systems
- **Tags over hierarchies**: Simple, flat organization using tags
- **Zero trust by default**: Explicit permissions required (Tailscale-inspired)
- **Convention over configuration**: Opinionated defaults with minimal setup required
- **CLI-first**: Everything manageable via command line
- **File-based source of truth**: Human-readable, VCS-friendly configuration
- **Incremental adoption**: Minimal initial setup, extensible as homelab grows
- **Explicit over automatic**: Manual profile assignment prevents accidental changes
- **Pure declarative configs**: No conditional logic in YAML/JSON/TOML files

## Directory Structure

**Structure focused on inventory, profiles, and delegation:**

```
homelab/
├── .appa/
│   ├── config.toml                 # User preferences & LDAP config
│   ├── plugins.toml                # Plugin configuration and settings
│   └── templates/                  # System templates
│       ├── server.yaml
│       └── device.yaml
├── modules/                        # Configuration modules with multiple handler support
│   ├── web-server/
│   │   ├── appa.yaml              # Module definition
│   │   ├── configuration.nix      # Nix implementation
│   │   ├── playbook.yml           # Ansible implementation
│   │   └── compose.yml            # Docker implementation
│   ├── database/
│   │   ├── appa.yaml
│   │   ├── flake.nix              # Nix flake
│   │   └── deployment.yaml        # Kubernetes implementation
│   └── monitoring/
│       ├── appa.yaml
│       └── docker-compose.yml
├── profiles/                       # Configuration profiles (explicitly assigned)
│   ├── web-server.yaml            # Web server configuration
│   ├── database.yaml              # Database configuration
│   ├── production.yaml            # Production environment settings
│   ├── monitoring.yaml            # Monitoring configuration
│   ├── security-baseline.yaml     # Security baseline
│   └── vm-tools.yaml              # VM-specific tools
├── systems/                        # System definitions with explicit profile assignment
│   ├── web-01.yaml                # explicitly assigns: [web-server, production, monitoring]
│   ├── db-01.yaml                 # explicitly assigns: [database, production, backup]
│   └── dev-web.yaml               # explicitly assigns: [web-server, development]
├── policy.yaml                     # Single ACL file (references LDAP users/groups)
├── secrets.yaml                    # Encrypted secrets managed by SOPS
├── maintainers.yaml                # Centralized maintainer registry
└── inventory.yaml                  # Global index
```

## CLI Command Structure

**Consistent `<object> <verb> [name]` pattern for object-specific context:**
- **Object operations**: `appa <object> <verb> [name]` - All operations under object type subcommands
- **Positional arguments**: Object names are positional arguments, not subcommands

### Secrets Management
```bash
# Secrets operations (uses SOPS plugin by default)
appa secrets show                    # List all secret keys
appa secrets get ldap/bind_password  # Get specific secret value
appa secrets set ldap/bind_password  # Set secret (prompts for value)
appa secrets edit                    # Open secrets.yaml in $EDITOR
appa secrets encrypt                 # Re-encrypt with new keys
appa secrets decrypt                 # Decrypt for editing
appa secrets rotate                  # Generate new secret values
```

### Plugin Management
```bash
# Plugin operations
appa plugin list                     # List installed plugins
appa plugin show sops               # Show plugin details and status
appa plugin install bmc             # Install plugin from repository
appa plugin update                  # Update all plugins (default)
appa plugin update sops             # Update specific plugin
appa plugin enable proxmox          # Enable plugin
appa plugin disable docker          # Disable plugin
appa plugin configure sops          # Configure plugin settings
appa plugin test ansible            # Test plugin functionality
```


### Object Operations: `appa <object> <verb> [name]`
```bash
# System operations with system-specific context
appa system add web-01 --type=server --env=prod --handler=nix
appa system remove web-01
appa system list --env=prod --type=server
appa system show web-01              # Show basic object properties
appa system set web-01 --env=staging --handler=ansible
appa system edit web-01
appa system validate web-01
appa system deploy web-01 --validate

# Profile operations with profile-specific context
appa profile add nginx-proxy --description="Nginx reverse proxy"
appa profile list --tag=web
appa profile show nginx-proxy
appa profile edit nginx-proxy
appa profile validate nginx-proxy

# Module operations with module-specific context
appa module add web-server --handler=nix --description="Web server config"
appa module list --handler=nix
appa module show web-server
appa module edit web-server
appa module validate web-server
```

### Object-Specific Operations: `appa <object> <verb> <name>`
```bash
# System-specific operations with name as positional argument
appa system set web-01 --env=staging
appa system edit web-01
appa system show web-01              # Show detailed system-specific info
appa system set web-01 --tag+=critical  # Add manual tag
appa system set web-01 --tag-=critical  # Remove manual tag

# Explicit profile management
appa system set web-01 --profile+=web-server   # Explicitly add profile
appa system set web-01 --profile-=monitoring   # Remove profile
appa system show web-01 --profiles             # Show assigned profiles

# Validation and deployment
appa system validate web-01          # Check against mandatory policies
appa system deploy web-01 --validate # Validate before deploying
appa system show web-01 --policies   # Show which policies apply
```

### Complete Object Operations
```bash
# Profile management
appa profile add nginx-proxy --description="Nginx reverse proxy"
appa profile list
appa profile show nginx-proxy       # Show basic object properties
appa profile edit nginx-proxy       # Edit specific profile
appa profile validate nginx-proxy   # Validate specific profile
appa profile show nginx-proxy --systems # Show systems using profile

# Module management
appa module add web-server --handler=nix --description="Web server config"
appa module list
appa module show web-server         # Show basic object properties
appa module edit web-server         # Edit specific module
appa module validate web-server     # Validate specific module
appa validate modules               # Validate all modules

# Policy management
appa policy list
appa policy show production-monitoring
appa validate policies

# Deployment operations
appa deploy --env=prod --validate
appa validate all
appa export inventory --backend=ansible
```

### Policy & Configuration
```bash
# Policy operations
appa policy edit
appa policy show

# LDAP sync operations (one-way: appa → LDAP)
appa ldap sync --dry-run
appa ldap sync groups
appa ldap sync all
appa ldap status

# Configuration
appa config set --key=format --value=yaml
appa config set --key=ldap.server --value=ldap.homelab.local
appa config show

# Source control (use git directly - appa files are git-friendly)
git add .
git commit -m "Add new web server"
git push
git status
```

## Object Types

Appa manages several types of objects in your homelab:

- **[Systems](system.md)** - Physical and virtual machines
- **[Profiles](profile.md)** - Reusable configuration templates
- **[Modules](module.md)** - Backend-specific configuration code
- **[Policies](policy.md)** - Access control and enforcement rules
- **[Secrets](secret.md)** - Encrypted configuration values

## Plugin Architecture

Appa uses plugins to extend functionality for specific backends and services:

- **[SOPS Plugin](plugins/sops.md)** - Encrypted secrets management
- **[BMC Plugin](plugins/bmc.md)** - Bare metal provisioning and power management
- **[PXE Plugin](plugins/pxe.md)** - Network boot and OS installation
- **[LDAP Plugin](plugins/ldap.md)** - LDAP/FreeIPA integration for infrastructure groups
- **[Proxmox Plugin](plugins/proxmox.md)** - Hypervisor and VM management
- **[Nix Plugin](plugins/nix.md)** - NixOS configuration management
- **[Ansible Plugin](plugins/ansible.md)** - Playbook-based configuration
- **[Docker Plugin](plugins/docker.md)** - Container orchestration

## Schema Definitions

### System Schema

**System definition (systems/web-01.yaml):**
```yaml
# Basic system info
name: web-01
description: "Primary web server hosting company website"
type: server
env: prod
backend: nix

# Auto-generated tags: [server, prod, web]
# Manual tags can be added:
additional_tags: [critical, public-facing]

# Hardware and network info (provided to external tools)
hardware:
  cpu: 4
  memory: 8GB
network:
  ip: 10.0.1.10
  domain: homelab.local

# Backend-specific config (passed to external tools)
backend_config:
  channel: "nixos-unstable"
  flake_ref: "github:myorg/homelab-nix"
```

### Profile Schema

**Tag-based profile definition (profiles/web-server.yaml):**
```yaml
name: web-server
description: "Web server configuration"

# What to apply and how (delegates to external tools)
configurations:
  # Systems with nix handler
  - handler: nix
    module: "web-server"

  # Systems with docker handler
  - handler: docker
    module: "web-server"

  # Systems with ansible handler
  - handler: ansible
    module: "web-server"
```

**Production profile (profiles/production.yaml):**
```yaml
name: production
description: "Production environment settings"

configurations:
  # Apply monitoring to all systems
  - handler: all
    module: "monitoring"

  # Nix systems get additional hardening
  - handler: nix
    module: "security-hardening"

  # Docker systems get production settings
  - handler: docker
    module: "production-settings"

  # Ansible systems get hardening playbook
  - handler: ansible
    module: "security-hardening"
```

### Access Control Schema

**Single policy file (policy.yaml) - Tailscale-inspired:**
```yaml
# Groups (like Tailscale)
groups:
  family: [alice, bob]
  admins: [alice]
  services: [backup, monitoring]

# Tag owners (who can assign tags)
tag_owners:
  tag:prod: [group:admins]
  tag:dev: [group:family]
  tag:service: [group:admins]

# Profile enforcement policies (mandatory profiles)
profile_policies:
  # All production systems must have monitoring
  - name: "production-monitoring"
    tags: [tag:prod]
    required_profiles: [monitoring, security-baseline]

  # All servers must have backup configuration
  - name: "server-backup"
    tags: [tag:server]
    required_profiles: [backup]

  # All public-facing systems need hardening
  - name: "public-hardening"
    tags: [tag:public-facing]
    required_profiles: [security-hardening, monitoring]

# Access rules (Tailscale-inspired)
acls:
  # Everyone can access their own devices
  - action: allow
    src: [autogroup:owner]
    dst: [autogroup:owned:*]

  # Admins can access everything
  - action: allow
    src: [group:admins]
    dst: [tag:*:*]

  # Family can access dev systems
  - action: allow
    src: [group:family]
    dst: [tag:dev:*]

  # Services can access what they need
  - action: allow
    src: [backup]
    dst: [tag:server:22]  # SSH to servers for backup

  # Default mesh connectivity for basic services
  - action: allow
    src: [tag:*]
    dst: [tag:*:53,67,68]  # DNS, DHCP
```

## Opinionated Mesh Defaults

**Core Tags (Auto-Applied):**
```yaml
# System type tags (automatically assigned)
tag:server      # Physical/VM servers
tag:hypervisor  # Virtualization hosts (Proxmox, VMware, etc.)
tag:device      # Network devices, IoT, etc.
tag:workstation # Laptops, desktops
tag:service     # Service accounts, automation

# Environment tags
tag:prod        # Production systems
tag:dev         # Development/testing
tag:mgmt        # Management/monitoring

# Service tags (derived from system purpose)
tag:web         # Web servers
tag:db          # Databases
tag:storage     # NAS, backup systems
tag:network     # Switches, routers, APs
tag:proxy       # Reverse proxies
tag:vpn         # VPN services
tag:containers  # Container hosts

# Plugin-provided tags
tag:proxmox-host    # Provides Proxmox hypervisor capabilities
tag:bare-metal      # Physical hardware managed by BMC
tag:encrypted       # Systems using encrypted storage/secrets
```

**Mesh Network Assumptions:**
```yaml
# Default mesh networking (like Tailscale)
mesh_network:
  default_subnet: 10.0.0.0/16      # Assumes private mesh
  auto_discovery: true             # mDNS/Bonjour
  direct_connectivity: true        # Peer-to-peer when possible

# Standard service ports (always allowed)
core_services:
  dns: 53
  dhcp: 67,68
  ntp: 123
  ssh: 22                          # Controlled by ACLs

# Auto-tagging rules
auto_tags:
  server: [tag:server, tag:ssh]
  device: [tag:device]
  workstation: [tag:workstation, tag:ssh]
  service: [tag:service, tag:automated]
```

### Template System

**Simplified server template (.appa/templates/server.yaml):**
```yaml
name: "{{ name }}"
type: server
env: "{{ env | default('dev') }}"
function: "{{ function | prompt('Function (web/db/storage)') }}"

hardware:
  cpu: "{{ cpu | default(2) }}"
  memory: "{{ memory | default('4GB') }}"

network:
  ip: "{{ ip | prompt('IP address') }}"
  mesh_name: "{{ name }}.home"
```

## Key Simplifications

**What we removed for simplicity:**

1. **Complex network modeling** → Assume mesh connectivity
2. **Folder-per-system** → Single YAML file per system
3. **Separate interface files** → Network info in system definition
4. **Complex RBAC system** → Tailscale-inspired tags and groups
5. **Multiple policy files** → Single policy.yaml file
6. **Hierarchical groups** → Flat group membership
7. **Complex relationships** → Simple tag-based organization

## Implementation Strategy

### Core Rust Crates
- `serde` + `serde_yaml` - YAML serialization (default format)
- `clap` - CLI with subcommands and opinionated defaults
- `git2` - Git operations
- `handlebars` - Simple template engine
- `anyhow` - Error handling

### Plugin Architecture
- **BMC Plugin** - Bare metal provisioning via IPMI/Redfish
- **SOPS Plugin** - Secrets management with age/GPG encryption
- **PXE Plugin** - Network boot and OS installation
- **LDAP Plugin** - LDAP/FreeIPA integration for infrastructure groups
- **Proxmox Plugin** - VM lifecycle and hypervisor management
- **Nix Plugin** - NixOS configuration management
- **Ansible Plugin** - Playbook-based configuration
- **Docker Plugin** - Container orchestration

### Key Components

1. **CLI Manager** - Command structure with opinionated defaults
2. **File Manager** - Basic CRUD on YAML files
3. **Tag Engine** - Auto-tagging based on system type/function
4. **Policy Engine** - Tailscale-inspired ACL validation
5. **Plugin Manager** - Load, configure, and manage plugins

### Development Phases

**Phase 1: MVP**
- Basic system CRUD with auto-tagging
- Simple policy validation
- VCS-friendly diffable file formats

**Phase 2: Polish**
- Template system
- Advanced querying by tags
- Policy conflict detection

## Default Configuration Format

**Recommended Default: YAML**

YAML is recommended as the default format for the following reasons:

### Why YAML?
YAML is the recommended default format. See [Design Decisions](dd.md) for detailed rationale and format comparisons.


## Security Considerations

- SSH key fingerprint tracking and validation
- Certificate expiration monitoring
- Access rule validation and conflict detection
- Audit trail through Git history
- Separation of sensitive data (keys in separate files)
- Template validation to prevent injection attacks

## Future Enhancements

- Web UI for visual management
- Integration with configuration management tools (Ansible, Salt)
- Real-time system status monitoring
- Automated compliance checking
- Backup and disaster recovery procedures
- Multi-tenant support for larger organizations