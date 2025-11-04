# System Commands

System operations manage physical and virtual machines in your homelab infrastructure.

## System Operations

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
appa system validate --all
appa system deploy web-01 --validate
appa system test web-01 --module=web-server
```

## System Types

- **server** - Physical or virtual server
- **vm** - Virtual machine
- **container** - Container instance
- **network** - Network device (router, switch)
- **storage** - Storage device (NAS, SAN)

## Common Flags

- `--type=TYPE` - System type (server, vm, container, network, storage)
- `--env=ENV` - Environment (prod, staging, dev, test)
- `--handler=HANDLER` - Configuration handler (nix, ansible, docker)
- `--tag+=TAG` - Add tag
- `--tag-=TAG` - Remove tag
- `--profile+=PROFILE` - Add profile
- `--profile-=PROFILE` - Remove profile
- `--maintainer+=USER` - Add maintainer
- `--maintainer-=USER` - Remove maintainer
- `--force` - Force operation without confirmation

## See Also

- [Managing Systems](../01-user-guide/managing-systems.md) - Concepts and workflows
- [Profile Commands](cli-profile.md) - Profile management
- [Module Commands](cli-module.md) - Module management