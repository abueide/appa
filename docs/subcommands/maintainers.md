# Maintainers Subcommand

The `appa maintainer` subcommand manages the centralized maintainer registry and maintainer assignments.

## Overview

Maintainers are defined once in `maintainers.yaml` and referenced by handle in object definitions. This provides a single source of truth for contact information that automatically updates across all objects.

## Commands

### List Maintainers

```bash
# List all registered maintainers
appa maintainer list

# Show detailed information for specific maintainer
appa maintainer show alice
appa maintainer show platform-team
```

### Add Maintainers

```bash
# Add new maintainer interactively
appa maintainer add charlie

# Add maintainer with specific details
appa maintainer add charlie \
  --name="Charlie Wilson" \
  --email="charlie@homelab.local" \
  --github="charliewilson"

# Add team maintainer
appa maintainer add devops-team \
  --name="DevOps Team" \
  --email="devops@homelab.local" \
  --slack="#devops"
```

### Update Maintainers

```bash
# Update maintainer contact information
appa maintainer edit alice
appa maintainer set alice --website="https://alice.blog"
appa maintainer set alice --slack="@alice-new"

# Add contact method
appa maintainer add-contact alice --discord="alice#1234"

# Remove contact method
appa maintainer remove-contact alice --github
```

### Remove Maintainers

```bash
# Remove maintainer (with dependency checking)
appa maintainer remove charlie

# Force removal (ignores objects that reference this maintainer)
appa maintainer remove charlie --force
```

### Validate Maintainers

```bash
# Validate maintainer registry
appa maintainer validate

# Check for orphaned maintainer references
appa maintainer check-references

# Show which objects reference a maintainer
appa maintainer references alice
```

## Maintainer Registry Format

### File Location
- **Primary**: `maintainers.yaml` in homelab root
- **Backup**: `.appa/maintainers.yaml` (if primary not found)

### Registry Schema

```yaml
format_version: "1.0"

maintainers:
  # Individual maintainer
  alice:
    name: "Alice Johnson"
    email: "alice@homelab.local"
    website: "https://alice.dev"           # Optional
    contact:                               # Optional
      github: "alicejohnson"
      bsky: "alice.bsky.social"
      matrix: "@alice:homelab.local"
      slack: "@alice"

  # Team maintainer
  platform-team:
    name: "Platform Team"
    email: "platform@homelab.local"
    website: "https://wiki.homelab.local/teams/platform"
    contact:
      slack: "#infrastructure"
      oncall: "https://pagerduty.com/schedules/platform"
      mattermost: "@platform-team"
```

### Required Fields
- **handle**: Unique identifier (key in YAML)
- **name**: Full name or team name
- **email**: Primary contact email

### Optional Fields
- **website**: Personal website, blog, or team page
- **contact**: Arbitrary key-value pairs for platform-specific handles

## Object References

Objects reference maintainers by handle instead of embedding full contact info:

### Current Pattern (Centralized)
```yaml
# In systems/web-01.yaml
maintainers:
  - "alice"           # References maintainers.yaml entry
  - "platform-team"   # References maintainers.yaml entry
```

### Old Pattern (Embedded)
```yaml
# In systems/web-01.yaml (deprecated)
maintainers:
  - name: "Alice Johnson"
    email: "alice@homelab.local"
    # ... full contact info duplicated everywhere
```

## CLI Examples

### Basic Operations
```bash
# Add yourself as a maintainer
appa maintainer add $(whoami) \
  --name="Your Name" \
  --email="$(whoami)@homelab.local"

# Show your maintainer info
appa maintainer show $(whoami)

# List all systems you maintain
appa maintainer references $(whoami)
```

### Team Management
```bash
# Add team maintainer
appa maintainer add devops \
  --name="DevOps Team" \
  --email="devops@company.com" \
  --slack="#devops-alerts"

# Assign team as maintainer to critical systems
appa web-01 add-maintainer devops
appa db-01 add-maintainer devops
```

### Contact Updates
```bash
# Update Slack handle (automatically updates all references)
appa maintainer set alice --slack="@alice-new-handle"

# Add new contact method
appa maintainer add-contact alice --matrix="@alice:matrix.org"

# Validate all contact info is accessible
appa maintainer validate --check-connectivity
```

### Maintenance Operations
```bash
# Find maintainers with no assigned objects
appa maintainer list --unused

# Find objects with no maintainers
appa validate --check-maintainers

# Export maintainer contact list
appa maintainer export --format=csv > contacts.csv
```

## Integration with Other Commands

### System Commands
```bash
# Add/remove maintainers from systems
appa web-01 add-maintainer alice
appa web-01 remove-maintainer bob
appa web-01 list-maintainers

# Show systems maintained by specific person
appa system list --maintainer=alice
```

### Profile and Module Commands
```bash
# Assign maintainers to profiles
appa nginx-proxy add-maintainer platform-team

# Assign maintainers to modules
appa web-server add-maintainer alice

# Show all objects maintained by team
appa list all --maintainer=platform-team
```

## Validation Rules

### Handle Validation
- Must match pattern: `^[a-z0-9-]+$`
- Length between 3-63 characters
- Must be globally unique
- Cannot conflict with appa command names

### Contact Validation
- Email must be valid format
- Website must be valid URL
- Contact handles validated per platform
- No duplicate handles across maintainers

### Reference Validation
- All object maintainer references must exist in registry
- Orphaned maintainers generate warnings
- Objects without maintainers generate warnings

## Migration from Embedded Format

```bash
# Migrate existing embedded maintainer info to registry
appa maintainer migrate --from-embedded

# Preview migration without changes
appa maintainer migrate --dry-run

# Migrate specific objects only
appa maintainer migrate --objects="web-01,db-01"
```

## Best Practices

### Naming Conventions
- **Individuals**: Use first name or username: `alice`, `bob`, `charlie`
- **Teams**: Use descriptive names: `platform-team`, `security-team`, `network-ops`
- **Avoid**: Generic names like `admin`, `user`, `team`

### Contact Information
- **Primary Email**: Always provide reliable primary email
- **Multiple Channels**: Include multiple contact methods for availability
- **Current Info**: Keep contact information up to date
- **Team Contacts**: Use shared channels for team maintainers

### Assignment Strategy
- **Individual Objects**: Assign to person who created/owns the configuration
- **Shared Objects**: Assign to relevant team
- **Critical Systems**: Always assign multiple maintainers
- **Documentation**: Include team maintainers for institutional knowledge