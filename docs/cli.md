# CLI Commands

The `appa` CLI provides the main entry point for homelab infrastructure management. This page covers the root command structure and links to detailed subcommand documentation.

## Overview

The appa CLI uses a consistent `<object> <verb> <name>` command structure where object names are positional arguments. This provides object-specific context and allows tailored operations with object-specific defaults.

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

## Object Operations

All operations follow the `appa <object> <verb>` pattern, allowing object-specific context and tailored operations.

### System Operations

```bash
# Create new system
appa system add web-01 --type=server --env=prod --handler=nix

# List systems
appa system list
appa system list --env=prod --type=server
appa system list --tag=nginx --maintainer=alice

# Show system information
appa system show web-01
appa system show web-01 --format=json

# Remove system
appa system remove web-01
appa system remove web-01 --force

# Modify system properties
appa system set web-01 --env=staging --handler=ansible

# Edit system configuration
appa system edit web-01

# Tag management
appa system set web-01 --tag+=critical
appa system set web-01 --tag-=testing
appa system show web-01 --tags

# Profile management
appa system set web-01 --profile+=nginx-proxy
appa system set web-01 --profile-=monitoring
appa system show web-01 --profiles

# Maintainer management
appa system set web-01 --maintainer+=alice
appa system set web-01 --maintainer-=bob
appa system show web-01 --maintainers

# Validation and deployment
appa system validate web-01
appa system deploy web-01 --validate
appa system test web-01 --module=web-server
```

### Profile Operations

```bash
# Create new profile
appa profile add nginx-proxy --description="Nginx reverse proxy configuration"

# List profiles
appa profile list
appa profile list --tag=web --unused

# Show profile information
appa profile show nginx-proxy

# Remove profile
appa profile remove nginx-proxy

# Edit profile configuration
appa profile edit nginx-proxy

# Validate profile
appa profile validate nginx-proxy

# Show which systems use this profile
appa profile show nginx-proxy --systems

# Module management
appa profile set nginx-proxy --module+=web-server
appa profile set nginx-proxy --module-=old-web-config
appa profile show nginx-proxy --modules
```

### Module Operations

```bash
# Create new module
appa module add web-server --handler=nix --description="Web server configuration"

# List modules
appa module list
appa module list --handler=nix

# Show module information
appa module show web-server

# Remove module
appa module remove web-server

# Edit module definition
appa module edit web-server

# Validate module structure
appa module validate web-server

# Test module against systems
appa module test web-server --system=web-01
appa module test web-server --dry-run

# Show module usage
appa module show web-server --profiles
appa module show web-server --systems
```

## Global Operations

### Validation

```bash
# Validate everything
appa validate all

# Validate specific object types
appa validate systems
appa validate profiles
appa validate modules
appa validate policies

# Validate with specific checks
appa validate --check=references     # Check cross-references
appa validate --check=maintainers    # Check maintainer assignments
appa validate --check=secrets        # Check secret references
```

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

### Plugin Configuration

```bash
# List available plugins
appa plugin list

# Show plugin information
appa plugin show sops
appa plugin info nix

# Install and manage plugins
appa plugin install bmc
appa plugin enable ansible
appa plugin disable docker
appa plugin configure proxmox
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
appa validate systems --env=prod

# Deploy to production
appa deploy --env=prod --validate

# Show production statistics
appa stats --env=prod
```

### Maintenance Operations

```bash
# Find systems without maintainers
appa validate --check=maintainers

# Find unused profiles
appa profile list --unused

# Check for broken references
appa validate --check=references

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
appa validate all --verbose

# Fix validation issues
appa validate --fix=references
appa validate --fix=maintainers
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
appa web-01 deploy
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

- **[Secrets](subcommands/secrets.md)** - Encrypted secrets management with SOPS
- **[Maintainers](subcommands/maintainers.md)** - Centralized maintainer registry management

### Planned Subcommands

- **Plugins** - Plugin installation and management
- **Config** - Global configuration management
- **Policy** - Access control and policy validation
- **Sync** - LDAP synchronization operations
- **Deploy** - System deployment and validation
- **Modules** - Module management and testing