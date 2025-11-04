# Policies

Policies define access control rules and mandatory profile enforcement across your homelab.

## Schema

The single `policy.yaml` file contains all policies:

### Groups
```yaml
groups:
  family: [alice, bob]
  admins: [alice]
  services: [backup, monitoring]
```

### Tag Ownership
```yaml
tag_owners:
  tag:prod: [group:admins]
  tag:dev: [group:family]
  tag:service: [group:admins]
```

### Profile Enforcement Policies
```yaml
profile_policies:
  - name: "production-monitoring"
    tags: [tag:prod]
    required_profiles: [monitoring, security-baseline]

  - name: "server-backup"
    tags: [tag:server]
    required_profiles: [backup]
```

### Access Control Lists (ACLs)
```yaml
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
```

## Design Principles

- **Single File**: All policies in `policy.yaml` for centralized management
- **Tailscale-Inspired**: ACL syntax similar to Tailscale for familiarity
- **Tag-Based**: Rules apply to systems based on tags
- **Explicit Groups**: Reference LDAP/FreeIPA users and groups
- **Zero Trust**: Explicit permissions required

## Policy Types

### Profile Enforcement
Mandatory profiles based on system tags:
```yaml
profile_policies:
  - name: "public-hardening"
    tags: [tag:public-facing]
    required_profiles: [security-hardening, monitoring]
```

### Access Control
Network access rules:
```yaml
acls:
  - action: allow
    src: [group:engineering]
    dst: [tag:dev:*]
```

### Tag Ownership
Who can assign specific tags:
```yaml
tag_owners:
  tag:critical: [group:admins]
```

## CLI Commands

### Policy Management
```bash
appa policy list
appa policy show production-monitoring  # Basic properties
appa policy edit production-monitoring  # Edit specific policy
appa policy validate production-monitoring # Validate specific policy
appa validate policies  # Validate all policies
```

### Policy Validation
```bash
appa kyoshi show policies           # Show policies affecting a system
appa validate all                   # Check all systems against policies
```

## Examples

See the [sample homelab](../sample/homelab/policy.yaml) for complete policy examples.