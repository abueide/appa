# Systems

Systems represent physical or virtual machines in your homelab infrastructure.

## Schema

### Basic Properties
```yaml
name: kyoshi                         # Globally unique identifier
description: "Nginx reverse proxy server for routing traffic to internal services"
type: server                         # server, hypervisor, device, workstation, service
env: prod                           # prod, dev, mgmt
backend: nix                        # Primary configuration management backend
```

### Hardware Configuration
```yaml
hardware:
  cpu: 2
  memory: 2GB
  disk: 20GB
  architecture: x86_64
  # BMC configuration for bare metal systems
  bmc:
    ip: 10.0.0.5
    mac: "aa:bb:cc:dd:ee:ff"
    vendor: "supermicro"
```

### Network Configuration
```yaml
network:
  ip: 10.0.1.10
  domain: avatar
  fqdn: kyoshi.avatar
```

### Tags and Profiles
```yaml
# Auto-generated tags based on type/env
# Manual tags for specific functionality
additional_tags: [nginx, reverse-proxy, public-facing]

# Explicitly assigned profiles (no automatic assignment)
profiles: [nginx-proxy, production, ssh-access]
```

### Backend-Specific Configuration
```yaml
# Configuration passed to the backend plugin
backend_config:
  channel: "nixos-23.11"

# Plugin-specific sections
proxmox:
  vm:
    vm_id: 101
    template: "nixos-template"
    node: "iroh"

bmc:
  install_image: "proxmox-ve-8.1-iso"
  boot_mode: "uefi"
  post_install:
    ssh_keys:
      - "secrets://ssh/admin"
```

### Services
```yaml
# Services this system provides
services:
  - name: nginx
    port: 80
    public: true
  - name: nginx-ssl
    port: 443
    public: true
```

## Auto-Generated Tags

Systems automatically receive tags based on their properties:
- **Type tags**: `server`, `hypervisor`, `device`, `workstation`
- **Environment tags**: `prod`, `dev`, `mgmt`
- **Plugin tags**: `proxmox-host`, `bare-metal`, etc.

## CLI Commands

### System Operations
```bash
appa system add kyoshi --type=server --env=prod
appa system remove kyoshi
appa system list --env=prod --type=server
appa system show kyoshi              # Basic properties
appa system set kyoshi --env=staging --handler=ansible
appa system edit kyoshi              # Edit configuration
appa system validate kyoshi          # Validate configuration
appa system deploy kyoshi --validate # Deploy with validation

# Tag and profile management
appa system set kyoshi --tag+=nginx --tag+=reverse-proxy
appa system set kyoshi --profile+=nginx-proxy --profile+=production
appa system show kyoshi --tags --profiles --maintainers
```
appa kyoshi set --env=staging
appa kyoshi edit
appa kyoshi add --tag=critical
appa kyoshi rm --tag=critical
appa kyoshi add --profile=nginx-proxy
appa kyoshi rm --profile=monitoring
appa kyoshi show profiles
appa kyoshi validate
appa kyoshi deploy --validate
appa kyoshi show policies
```

## Examples

See the [sample homelab](../sample/homelab/systems/) for complete system examples.