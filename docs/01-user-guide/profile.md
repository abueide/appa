# Profiles

Profiles define reusable configuration templates that can be explicitly assigned to systems.

## Schema

### Basic Properties
```yaml
name: nginx-proxy
description: "Nginx reverse proxy configuration for routing traffic"
```

### Configuration Modules
```yaml
# Pure declarative - no conditional logic
configurations:
  # Nix-based systems
  - backend: nix
    module: "web-server-nix"

  # Docker systems
  - backend: docker
    module: "web-server-docker"

  # Ansible systems
  - backend: ansible
    module: "web-server-ansible"

  # Apply to all backends
  - backend: all
    module: "monitoring-common"
```

## Design Principles

- **Explicit Assignment**: Profiles must be manually assigned to systems
- **No Conditional Logic**: Pure declarative configuration in YAML
- **Backend Agnostic**: Same profile works across multiple backends
- **Module References**: Points to modules by name, not path

## Profile Assignment

Profiles are explicitly assigned in system definitions:
```yaml
# In systems/kyoshi.yaml
profiles: [nginx-proxy, production, ssh-access]
```

## CLI Commands

### Profile Management
```bash
appa profile add nginx-proxy
appa profile list
appa profile show nginx-proxy       # Basic properties
appa profile edit nginx-proxy       # Edit configuration
appa profile validate nginx-proxy   # Validate profile
appa profile show nginx-proxy --systems # Show systems using profile
```

### Profile Assignment
```bash
appa system set kyoshi --profile+=nginx-proxy
appa system set kyoshi --profile-=nginx-proxy
appa system show kyoshi --profiles
```

## Mandatory Profiles

Profiles can be enforced through policies in `policy.yaml`:
```yaml
profile_policies:
  - name: "production-monitoring"
    tags: [tag:prod]
    required_profiles: [monitoring, security-baseline]
```

## Examples

See the [sample homelab](../sample/homelab/profiles/) for complete profile examples.