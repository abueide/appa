# Appa Objects

Appa manages infrastructure through five core object types. All objects share common properties and follow consistent patterns for identification, organization, and lifecycle management.

## Object Types

- **[Systems](system.md)** - Physical and virtual machines
- **[Profiles](profile.md)** - Reusable configuration templates
- **[Modules](module.md)** - Backend-specific configuration code
- **[Policies](policy.md)** - Access control and enforcement rules
- **[Secrets](secret.md)** - Encrypted configuration values

## Common Properties

All Appa objects share a set of standard properties for identification and metadata.

### Required Properties

```yaml
# Universal identifier - must be globally unique across all object types
name: web-server-nix

# Human-readable description
description: "NixOS configuration for web servers with nginx and SSL"

# Schema version for compatibility and migration
format_version: "1.0"
```

### Optional Common Properties

```yaml
# Maintainers (references to maintainers.yaml)
maintainers:
  - "alice"           # References maintainers.yaml entry
  - "platform-team"   # References maintainers.yaml entry

# Arbitrary key-value metadata
metadata:
  owner: "infrastructure-team"
  cost_center: "engineering"
  documentation_url: "https://wiki.company.com/web-servers"


# Tags for organization and selection
tags:
  - "production"
  - "web"
  - "nginx"

# Dependencies on other appa objects
dependencies:
  - "nginx-base-config"  # Module dependency
  - "ssl-certificates"   # Secret dependency
```

## Naming Conventions

### Global Uniqueness
All object names must be globally unique across all object types. This enables:
- Direct object reference: `appa system show web-01`
- Unambiguous cross-references between objects
- Simplified CLI commands without path arguments

### Naming Rules
- **Format**: Lowercase alphanumeric with hyphens
- **Pattern**: `^[a-z0-9-]+$`
- **Length**: 3-63 characters
- **Examples**: `web-01`, `nginx-proxy`, `production-monitoring`

### Naming Conventions by Type
```yaml
# Systems: descriptive hostnames
systems: ["kyoshi", "aang", "toph", "iroh", "web-01", "db-primary"]

# Profiles: purpose-based names
profiles: ["nginx-proxy", "database-primary", "monitoring-agent", "ssh-access"]

# Modules: backend-purpose format
modules: ["web-server-nix", "database-ansible", "monitoring-docker"]

# Policies: descriptive policy names
policies: ["production-access", "ssh-baseline", "monitoring-required"]

# Secrets: path-like structure
secrets: ["ssh/admin", "database/root-password", "ssl/wildcard-cert"]
```

## File Formats

### Supported Formats
Appa supports multiple file formats with YAML as the default:

- **YAML** (`.yaml`, `.yml`) - Default format
- **JSON** (`.json`) - Programmatic access
- **TOML** (`.toml`) - Alternative human-readable format

### Format Selection
```yaml
# Global default in .appa/config.toml
format = "yaml"

# Per-object format (filename extension determines format)
systems/web-01.yaml    # YAML format
systems/web-02.json    # JSON format
systems/web-03.toml    # TOML format
```

## Schema Versioning

### Format Version Field
All objects include a `format_version` field for schema evolution:

```yaml
format_version: "1.0"  # Current schema version
```

### Version Compatibility
- **Backwards Compatibility**: Newer appa versions read older format versions
- **Migration**: Automatic migration to newer formats when saving
- **Validation**: Schema validation based on format version

### Version History
```yaml
# Version evolution example
"1.0": "Initial schema"
"1.1": "Added metadata field"
"1.2": "Added dependencies field"
```

## Object Lifecycle

### Creation
```bash
# Create from template
appa system add web-01 --template=server

# Create manually
appa system add web-01 --type=server --env=prod
```

### Modification
```bash
# Edit object properties
appa system set web-01 --env=staging
appa system set web-01 --tag+=critical

# Edit full configuration
appa system edit web-01
```

### Validation
```bash
# Validate individual object
appa system validate web-01

# Validate all objects
appa validate all

# Validate specific object type
appa validate systems
```

### Deletion
```bash
# Remove object (with dependency checking)
appa system remove web-01

# Force removal (ignores dependencies)
appa system remove web-01 --force
```

## Cross-References

### Reference Syntax
Objects reference other objects by name:

```yaml
# Profile references modules
configurations:
  - backend: nix
    module: "web-server-nix"  # References module by name

# System references profiles
profiles:
  - "nginx-proxy"     # References profile by name
  - "ssh-access"      # References profile by name

# Dependencies
dependencies:
  - "ssl-certificates"  # References secret by name
```

### Dependency Validation
- **Creation**: Check referenced objects exist
- **Deletion**: Prevent deletion of objects with dependents
- **Modification**: Validate references remain valid

## Object Organization

### Directory Structure
```
homelab/
├── systems/           # System definitions
│   ├── web-01.yaml
│   ├── db-01.yaml
│   └── dev-box.yaml
├── profiles/          # Profile definitions
│   ├── nginx-proxy.yaml
│   ├── database.yaml
│   └── monitoring.yaml
├── modules/           # Module implementations
│   ├── nix/
│   ├── ansible/
│   └── docker/
├── policy.yaml        # Single policy file
└── secrets.yaml       # Single secrets file
```

### File Naming
- **Systems and Profiles**: `{name}.{format}`
- **Modules**: Organized by backend in subdirectories
- **Policies and Secrets**: Single files (policy.yaml, secrets.yaml)

## Maintainers

### Centralized Maintainer System
Maintainers are defined once in `maintainers.yaml` and referenced by handle in object definitions:

```yaml
# In maintainers.yaml (centralized registry)
maintainers:
  alice:
    name: "Alice Johnson"
    email: "alice@company.com"
    website: "https://alice.dev"
    contact:
      github: "alicejohnson"
      bsky: "alice.bsky.social"
      matrix: "@alice:matrix.org"

  platform-team:
    name: "Platform Team"
    email: "platform@company.com"
    contact:
      slack: "#platform-team"
      oncall: "https://pagerduty.com/schedules/platform"
```

```yaml
# In object files (references by handle)
maintainers:
  - "alice"           # References alice entry in maintainers.yaml
  - "platform-team"   # References platform-team entry in maintainers.yaml
```

### Benefits of Centralized System
- **Single Source of Truth**: Contact info defined once, used everywhere
- **Automatic Updates**: Changing contact info updates all references
- **Consistency**: Prevents duplicate/conflicting maintainer information
- **Maintainer Management**: Dedicated commands for maintainer operations

### Maintainer Registry Commands
See [Maintainers Subcommand](subcommands/maintainers.md) for complete documentation:

```bash
# Manage maintainer registry
appa maintainer list
appa maintainer add alice --email="alice@company.com"
appa maintainer edit alice

# Assign maintainers to objects
appa system set web-01 --maintainer+=alice
appa system set web-01 --maintainer-=bob
appa system show web-01 --maintainers
```

### Usage Guidelines
- **Authorship**: Use VCS history for authorship tracking (e.g., `git log`, `hg log`)
- **Lifecycle**: Use VCS history for creation and modification timestamps
- **Maintenance**: Current responsibility for ongoing support and updates
- **Centralized Registry**: Define maintainers once in `maintainers.yaml`
- **Handle References**: Use maintainer handles, not embedded contact info
- **Team vs Individual**: Both individual and team maintainers supported

## Metadata and Documentation

### Metadata Field
Optional structured metadata for operational information:

```yaml
metadata:
  # Ownership and responsibility
  owner: "platform-team"
  maintainer: "alice@company.com"

  # Business context
  cost_center: "engineering"
  environment: "production"
  criticality: "high"

  # External references
  documentation_url: "https://wiki.company.com/systems/web-01"
  monitoring_url: "https://grafana.company.com/d/system-web-01"
  ticket_url: "https://jira.company.com/INFRA-123"

  # Custom fields
  backup_schedule: "daily"
  maintenance_window: "sunday-03:00-05:00"
```

### Description Guidelines
- **Purpose**: Explain what the object does
- **Context**: Provide relevant operational context
- **Length**: 1-2 sentences, under 200 characters
- **Examples**:
  - `"Primary web server hosting company website"`
  - `"Nginx reverse proxy configuration for internal services"`
  - `"Production monitoring and alerting profile"`

## Validation Rules

### Name Validation
- Must match pattern: `^[a-z0-9-]+$`
- Length between 3-63 characters
- Must be globally unique
- Cannot start or end with hyphen

### Reference Validation
- Referenced objects must exist
- Circular dependencies not allowed
- Module references must match backend type

### Format Validation
- Schema validation based on format_version
- Required fields must be present
- Field types must match schema
- Unknown fields generate warnings

## Example Object
```yaml
# Complete example showing all common properties
name: nginx-proxy
description: "Nginx reverse proxy configuration for routing traffic to internal services"
format_version: "1.0"

# Object-specific properties
configurations:
  - backend: nix
    module: "nginx-proxy-nix"
  - backend: docker
    module: "nginx-proxy-docker"

# Maintainers (references to maintainers.yaml)
maintainers:
  - "alice"
  - "platform-team"

# Common optional properties
metadata:
  owner: "platform-team"
  cost_center: "engineering"
  documentation_url: "https://wiki.company.com/nginx-proxy"

tags:
  - "production"
  - "web"
  - "proxy"

dependencies:
  - "ssl-certificates"
```