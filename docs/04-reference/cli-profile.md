# Profile Commands

Profile operations manage reusable configuration templates that can be assigned to systems.

## Profile Operations

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
appa profile validate --all

# Show which systems use this profile
appa profile show nginx-proxy --systems

# Module management
appa profile set nginx-proxy --module+=web-server
appa profile set nginx-proxy --module-=old-web-config
appa profile show nginx-proxy --modules

# Tag management
appa profile set nginx-proxy --tag+=web
appa profile set nginx-proxy --tag-=legacy
appa profile show nginx-proxy --tags

# Test profile against systems
appa profile test nginx-proxy --system=web-01
appa profile test nginx-proxy --dry-run
```

## Profile Properties

- **name** - Unique profile identifier
- **description** - Human-readable description
- **modules** - List of modules to apply
- **tags** - Classification tags
- **handler** - Primary configuration handler

## Common Flags

- `--description=DESC` - Profile description
- `--module+=MODULE` - Add module
- `--module-=MODULE` - Remove module
- `--tag+=TAG` - Add tag
- `--tag-=TAG` - Remove tag
- `--handler=HANDLER` - Configuration handler
- `--unused` - Show only unused profiles
- `--systems` - Show systems using this profile
- `--modules` - Show modules in this profile

## See Also

- [Profile Guide](../01-user-guide/profile.md) - Detailed profile management
- [System Commands](cli-system.md) - System management
- [Module Commands](cli-module.md) - Module management