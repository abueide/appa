# Managing Systems

Systems are the foundation of your homelab infrastructure - representing physical servers, virtual machines, containers, and network devices that you need to configure and manage.

## Key Concepts

### What are Systems?

A **system** in Appa represents any manageable infrastructure component:
- **Physical servers** - Bare metal machines in your homelab
- **Virtual machines** - VMs running on hypervisors like Proxmox
- **Containers** - Docker containers or LXC instances
- **Network devices** - Routers, switches, access points
- **Storage systems** - NAS devices, storage clusters

### System Lifecycle

Systems go through a typical lifecycle:

1. **Definition** - Create the system configuration
2. **Provisioning** - Install OS and basic configuration
3. **Configuration** - Apply profiles and modules
4. **Deployment** - Make the system operational
5. **Maintenance** - Updates, changes, monitoring
6. **Decommissioning** - Clean removal from infrastructure

## Common Workflows

### Adding a New Server

When you get a new physical server, here's the typical workflow:

```bash
# 1. Add the system definition
appa system add web-03 --type=server --env=prod

# 2. Configure hardware details
appa system set web-03 --cpu=4 --memory=8GB --disk=100GB

# 3. Assign yourself as maintainer
appa system set web-03 --maintainer+=$(whoami)

# 4. Add necessary profiles
appa system set web-03 --profile+=base-server
appa system set web-03 --profile+=nginx-proxy
appa system set web-03 --profile+=monitoring

# 5. Validate configuration
appa system validate web-03

# 6. Deploy (this will run your configuration management)
appa system deploy web-03 --validate
```

### Environment Management

Organize systems by environment to prevent accidental changes:

```bash
# List all production systems
appa system list --env=prod

# Deploy only to staging first
appa deploy --env=staging --validate

# After testing, deploy to production
appa deploy --env=prod --validate
```

### Profile Assignment Strategy

Profiles let you group related configuration:

```bash
# Base profiles everyone needs
appa system set web-03 --profile+=base-server
appa system set web-03 --profile+=ssh-access

# Role-specific profiles
appa system set web-03 --profile+=nginx-proxy    # Web server role
appa system set web-03 --profile+=monitoring     # Observability

# Environment-specific profiles
appa system set web-03 --profile+=production     # Prod settings
```

### Tag Organization

Use tags for flexible grouping and automation:

```bash
# Functional tags
appa system set web-03 --tag+=web
appa system set web-03 --tag+=frontend
appa system set web-03 --tag+=critical

# Location tags
appa system set web-03 --tag+=rack-1
appa system set web-03 --tag+=datacenter-home

# Later, operate on tagged groups
appa system list --tag=web
appa deploy --tag=critical --dry-run
```

## System Types

Choose the right type for your use case:

### Servers
Physical or virtual machines running services:
```bash
appa system add db-01 --type=server --env=prod
appa system set db-01 --profile+=database-server
```

### Hypervisors
Machines that host virtual machines:
```bash
appa system add proxmox-01 --type=hypervisor --env=prod
appa system set proxmox-01 --profile+=proxmox-host
```

### Network Devices
Routers, switches, and other network infrastructure:
```bash
appa system add router-01 --type=device --env=prod
appa system set router-01 --profile+=openwrt-router
```

### Workstations
Development machines and user workstations:
```bash
appa system add laptop-alice --type=workstation --env=dev
appa system set laptop-alice --profile+=developer-setup
```

## Configuration Backends

Different systems use different configuration management tools:

### NixOS Systems
```bash
appa system add nix-server --type=server --backend=nix
# Uses modules in modules/nix/
```

### Ansible-Managed Systems
```bash
appa system add ansible-server --type=server --backend=ansible
# Uses playbooks in modules/ansible/
```

### Docker Containers
```bash
appa system add web-container --type=container --backend=docker
# Uses docker-compose files in modules/docker/
```

## Best Practices

### Naming Conventions
- Use consistent, descriptive names: `web-01`, `db-primary`, `router-main`
- Include purpose and instance number: `nginx-proxy-01`, `postgres-primary`
- Avoid changing names after deployment

### Environment Separation
- Always specify environment: `--env=prod`, `--env=staging`, `--env=dev`
- Test changes in staging before production
- Use environment-specific profiles for different settings

### Maintainer Assignment
- Always assign maintainers to systems you create
- Use team names for shared systems: `--maintainer+=web-team`
- Keep maintainer information current

### Profile Strategy
- Start with base profiles, add role-specific ones
- Keep profiles focused and reusable
- Test profile combinations before wide deployment

## Integration with Other Objects

### Profiles
Profiles contain the actual configuration applied to systems:
```bash
# Profiles define what software and settings to apply
appa system set web-01 --profile+=nginx-proxy
```

### Modules
Modules contain the backend-specific implementation:
```bash
# Profiles reference modules for different backends
# nginx-proxy profile might use modules/nix/nginx.nix
```

### Policies
Policies control who can access and modify systems:
```bash
# Policies enforce access rules
appa policy test --user=alice --system=web-01
```

### Secrets
Secrets provide encrypted values to system configurations:
```yaml
# Systems can reference secrets in their config
database_password: secrets://database.web.password
```

## Troubleshooting

### Validation Failures
```bash
# Check what's wrong with a system
appa system validate web-01 --verbose

# Common issues and fixes
appa system validate --all --check=maintainers  # Missing maintainers
appa system validate --all --check=references   # Broken profile references
```

### Deployment Issues
```bash
# Test deployment without making changes
appa system deploy web-01 --dry-run --verbose

# Check system status
appa system show web-01
appa system show web-01 --profiles --maintainers
```

## See Also

- **[System Commands Reference](../04-reference/cli-system.md)** - Complete command options
- **[Profiles](profile.md)** - Configuration templates
- **[Policies](policy.md)** - Access control
- **[Deployment Workflows](../00-getting-started/)** - Step-by-step guides