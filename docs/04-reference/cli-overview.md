# CLI Commands Overview

The `appa` CLI provides the main entry point for homelab infrastructure management. This page covers the root command structure and links to detailed command documentation for each object type.

## Command Structure

The appa CLI uses a consistent `<object> <verb> <name>` command structure where object names are positional arguments. This provides object-specific context and allows tailored operations with object-specific defaults.

```bash
appa <object> <verb> [name] [flags]
```

## Object Commands

### Core Infrastructure Objects
- **[System Commands](cli-system.md)** - Physical and virtual machine management
- **[Profile Commands](cli-profile.md)** - Reusable configuration template management
- **[Module Commands](cli-module.md)** - Backend-specific configuration management
- **[Plugin Commands](cli-plugin.md)** - Runtime extension management
- **[Policy Commands](cli-policy.md)** - Access control and enforcement rules
- **[Secret Commands](cli-secret.md)** - Encrypted credential management

## Flag Conventions

- **List operations**: Use `+=` to add items, `-=` to remove items
- **Property setting**: Use `--key=value` format
- **Information display**: Use `--key` flags to show specific information
- **No hyphenated verbs**: Use flags instead of hyphenated subcommands

Examples:
- `appa system set web-01 --tag+=critical --profile+=nginx-proxy`
- `appa system set web-01 --tag-=testing --env=staging`
- `appa system show web-01 --tags --profiles --maintainers`

## Global Options

```bash
# Common flags available for all commands
appa [global-options] <command> [command-options]

--config PATH         # Path to config file (default: .appa/config.toml)
--format FORMAT       # Output format: yaml, json, table (default: yaml)
--verbose, -v         # Enable verbose output
--quiet, -q          # Suppress non-essential output
--dry-run            # Show what would be done without making changes
--help, -h           # Show help for command
--version            # Show version information
```

## Global Operations

### Deployment

```bash
# Deploy to specific environment
appa deploy --env=prod --validate

# Deploy specific systems
appa deploy --systems=web-01,db-01

# Deploy with handlers
appa deploy --handler=nix
appa deploy --handler=ansible

# Dry run deployment
appa deploy --dry-run --verbose
```

### Export and Import

```bash
# Export inventory for external tools
appa export inventory --backend=ansible
appa export inventory --backend=terraform
appa export inventory --format=json

# Export specific object types
appa export systems --env=prod
appa export profiles --tag=web

# Import from external sources
appa import systems --from-file=systems.yaml
appa import --from-ansible-inventory=hosts.ini
```

## Informational Commands

### Version and Help

```bash
# Show version information
appa version
appa --version

# Show general help
appa help
appa --help

# Show help for specific commands
appa help secrets
appa secrets --help
appa web-01 --help
```

### Status and Info

```bash
# Show overall status
appa status

# Show configuration information
appa info
appa config show

# Show statistics
appa stats
appa stats --by-env
appa stats --by-type
appa stats --by-maintainer
```

### Maintainer Operations

```bash
# List all registered maintainers
appa maintainer list
appa maintainer show alice

# Add new maintainer
appa maintainer add charlie \
  --name="Charlie Wilson" \
  --email="charlie@homelab.local" \
  --github="charliewilson"

# Add team maintainer
appa maintainer add devops-team \
  --name="DevOps Team" \
  --email="devops@homelab.local" \
  --slack="#devops"

# Update maintainer information
appa maintainer set alice --slack="@alice-new"
appa maintainer add-contact alice --discord="alice#1234"

# Validate maintainers
appa maintainer validate
appa maintainer check-references
appa maintainer references alice

# Remove maintainer
appa maintainer remove charlie
```

## Configuration Commands

### Global Configuration

```bash
# Show current configuration
appa config show

# Set configuration values
appa config set format yaml
appa config set ldap.server ldap.homelab.local
appa config set default_handler nix

# Reset configuration
appa config reset
appa config reset ldap.server
```

## Common Workflows

### Initial Setup

```bash
# Initialize new homelab
appa init --name="My Homelab"

# Add yourself as maintainer
appa maintainer add $(whoami) --name="Your Name" --email="your@email.com"

# Configure basic settings
appa config set format yaml
appa config set default_handler nix
```

### Adding New System

```bash
# Create system
appa system add web-02 --type=server --env=prod --handler=nix

# Assign maintainer
appa system set web-02 --maintainer+=$(whoami)

# Add profiles
appa system set web-02 --profile+=nginx-proxy
appa system set web-02 --profile+=production
appa system set web-02 --profile+=ssh-access

# Validate and deploy
appa system validate web-02
appa system deploy web-02 --dry-run
appa system deploy web-02
```

### Environment Management

```bash
# List production systems
appa system list --env=prod

# Validate production environment
appa system validate --all --env=prod

# Deploy to production
appa deploy --env=prod --validate

# Show production statistics
appa stats --env=prod
```

### Maintenance Operations

```bash
# Find systems without maintainers
appa system validate --all --check=maintainers

# Find unused profiles
appa profile list --unused

# Check for broken references
appa system validate --all --check=references

# Clean up orphaned objects
appa cleanup --dry-run
appa cleanup --confirm
```

## Output Formats

### YAML (Default)

```bash
appa system show web-01
# Outputs structured YAML

appa system list --format=yaml
# Outputs list as YAML array
```

### JSON

```bash
appa system show web-01 --format=json
# Outputs structured JSON for programmatic use

appa system list --format=json | jq '.[] | select(.env == "prod")'
# Pipe to jq for processing
```

### Table

```bash
appa system list --format=table
# Outputs human-readable table

appa stats --format=table
# Statistics in tabular format
```

## Error Handling

### Validation Errors

```bash
# Show detailed validation errors
appa system validate --all --verbose

# Fix validation issues
appa system validate --all --fix=references
appa system validate --all --fix=maintainers
```

### Deployment Errors

```bash
# Debug deployment issues
appa deploy --debug --verbose

# Rollback on failure
appa rollback --to-previous
appa rollback --to-commit=abc123
```

### Common Issues

```bash
# Object not found
appa system show nonexistent-system
# Error: System 'nonexistent-system' not found

# Missing dependencies
appa profile remove nginx-proxy
# Error: Profile 'nginx-proxy' is used by systems: web-01, web-02

# Validation failures
appa system deploy web-01
# Error: System validation failed: missing required profile 'ssh-access'
```

## Environment Variables

```bash
# Configuration overrides
export APPA_CONFIG="/path/to/config.toml"
export APPA_FORMAT="json"
export APPA_VERBOSE="true"

# Plugin-specific variables
export APPA_SOPS_AGE_KEY="/path/to/age/key"
export APPA_ANSIBLE_INVENTORY="/path/to/inventory"

# Authentication
export APPA_LDAP_PASSWORD="secret"
export APPA_PROXMOX_TOKEN="secret"
```

## Exit Codes

- **0**: Success
- **1**: General error
- **2**: Validation error
- **3**: Configuration error
- **4**: Network/connectivity error
- **5**: Permission/authentication error
- **6**: Resource not found
- **7**: Dependency error

## Subcommand Reference

### Core Subcommands

- **[Secrets](cli-secret.md)** - Encrypted secrets management with SOPS
- **Maintainers** - Centralized maintainer registry management (see Maintainer Operations above)

### Planned Subcommands

- **Config** - Global configuration management
- **Policy** - Access control and policy validation
- **Sync** - LDAP synchronization operations
- **Deploy** - System deployment and validation
- **Modules** - Module management and testing