# LDAP Plugin

The LDAP plugin deploys infrastructure groups and policies from appa's source of truth to LDAP servers, providing centralized authentication and authorization integration.

## Overview

The LDAP plugin performs one-way synchronization from appa to LDAP servers, making appa the authoritative source for infrastructure access control. It creates and manages LDAP groups based on appa's policy definitions and system tags.

### Supported LDAP Servers

- **FreeIPA** - Primary target with full feature support
- **Active Directory** - Basic group synchronization
- **OpenLDAP** - Basic group synchronization

## Configuration

### Plugin Configuration

```toml
# In .appa/plugins.toml
[plugins.ldap]
enabled = true
server_type = "freeipa"  # freeipa, active_directory, openldap
server_url = "ldaps://ipa.homelab.local"
base_dn = "dc=homelab,dc=local"

# Authentication
bind_dn = "uid=appa-sync,cn=users,cn=accounts,dc=homelab,dc=local"
bind_password = "secrets://ldap/bind_password"

# Sync configuration
sync_mode = "groups_only"  # groups_only, users_and_groups
group_base = "cn=groups,cn=accounts,dc=homelab,dc=local"
user_base = "cn=users,cn=accounts,dc=homelab,dc=local"

# Group naming
group_prefix = "appa-"  # Creates groups like "appa-admins", "appa-prod-access"
group_suffix = ""

# Sync behavior
dry_run = false
delete_orphaned_groups = false
create_missing_users = false
```

### FreeIPA Integration

```toml
[plugins.ldap.freeipa]
# IPA-specific settings
kinit_principal = "appa-sync@HOMELAB.LOCAL"
keytab_file = "secrets://ldap/appa_sync_keytab"
realm = "HOMELAB.LOCAL"

# Group management
create_hostgroups = true  # Create hostgroups for system tags
create_hbacrules = true   # Create HBAC rules for access control
```

## Synchronization Strategy

### Groups from Policy

The plugin creates LDAP groups based on policy definitions:

```yaml
# From policy.yaml
groups:
  admins: [alice, bob]
  family: [alice, bob, charlie]
  services: [backup, monitoring]

# Creates LDAP groups:
# - cn=appa-admins,cn=groups,cn=accounts,dc=homelab,dc=local
# - cn=appa-family,cn=groups,cn=accounts,dc=homelab,dc=local
# - cn=appa-services,cn=groups,cn=accounts,dc=homelab,dc=local
```

### Tag-Based Groups

Automatically creates groups for system tags:

```yaml
# From systems with tags: [prod, web, server]
# Creates LDAP groups:
# - cn=appa-prod,cn=groups,cn=accounts,dc=homelab,dc=local
# - cn=appa-web,cn=groups,cn=accounts,dc=homelab,dc=local
# - cn=appa-server,cn=groups,cn=accounts,dc=homelab,dc=local
```

### FreeIPA Hostgroups

Creates hostgroups for infrastructure management:

```yaml
# System: web-01 with tags [prod, web, server]
# Creates FreeIPA hostgroup: appa-web-systems
# Adds host: web-01.homelab.local
```

## Commands

### Sync Operations

```bash
# Dry run sync (show what would change)
appa ldap sync --dry-run

# Sync groups only
appa ldap sync groups

# Full sync (groups and users if enabled)
appa ldap sync all

# Sync specific group
appa ldap sync group admins
```

### Group Management

```bash
# List LDAP groups managed by appa
appa ldap list groups

# Show group details
appa ldap show group appa-admins

# Create missing groups
appa ldap create missing-groups

# Clean up orphaned groups
appa ldap cleanup --dry-run
appa ldap cleanup --confirm
```

### Validation

```bash
# Validate LDAP configuration
appa ldap validate config

# Test LDAP connection
appa ldap test connection

# Compare appa policy with LDAP state
appa ldap diff

# Verify group memberships
appa ldap verify memberships
```

### Status and Monitoring

```bash
# Show sync status
appa ldap status

# Show last sync information
appa ldap info

# Monitor sync operations
appa ldap monitor
```

## Group Mapping

### Policy Groups

```yaml
# policy.yaml
groups:
  admins: [alice, bob]
  family: [alice, bob, charlie, diana]
  developers: [alice, charlie]
  services: [backup-service, monitoring-service]

# Maps to LDAP groups:
# cn=appa-admins,cn=groups,cn=accounts,dc=homelab,dc=local
#   - member: uid=alice,cn=users,cn=accounts,dc=homelab,dc=local
#   - member: uid=bob,cn=users,cn=accounts,dc=homelab,dc=local
```

### Tag-Based Groups

```yaml
# Automatically derived from system tags
systems:
  web-01: [prod, web, server]
  db-01: [prod, database, server]
  dev-box: [dev, workstation]

# Creates groups:
# cn=appa-prod,cn=groups,cn=accounts,dc=homelab,dc=local
# cn=appa-web,cn=groups,cn=accounts,dc=homelab,dc=local
# cn=appa-server,cn=groups,cn=accounts,dc=homelab,dc=local
# cn=appa-database,cn=groups,cn=accounts,dc=homelab,dc=local
# cn=appa-dev,cn=groups,cn=accounts,dc=homelab,dc=local
# cn=appa-workstation,cn=groups,cn=accounts,dc=homelab,dc=local
```

### Access Control Groups

```yaml
# Derived from ACL rules
acls:
  - action: allow
    src: [group:admins]
    dst: [tag:prod:*]

  - action: allow
    src: [group:family]
    dst: [tag:dev:*]

# Creates access groups:
# cn=appa-prod-access,cn=groups,cn=accounts,dc=homelab,dc=local
# cn=appa-dev-access,cn=groups,cn=accounts,dc=homelab,dc=local
```

## FreeIPA Integration

### Hostgroup Management

```bash
# Create hostgroups for system organization
appa ldap sync hostgroups

# Show hostgroup mapping
appa ldap show hostgroups

# Add systems to appropriate hostgroups
appa ldap update hostgroups
```

### HBAC Rules

```bash
# Create HBAC rules from appa policies
appa ldap sync hbac

# Show HBAC rule mapping
appa ldap show hbac

# Test HBAC access
appa ldap test hbac alice web-01.homelab.local ssh
```

### Service Integration

```yaml
# FreeIPA services for appa-managed systems
services:
  - name: "HTTP/web-01.homelab.local"
    system: "web-01"
    profiles: ["nginx-proxy"]

  - name: "postgres/db-01.homelab.local"
    system: "db-01"
    profiles: ["database"]
```

## User Management

### User Discovery

```bash
# Discover users from LDAP
appa ldap discover users

# Import users into appa policy
appa ldap import users --group=family

# Show user mapping
appa ldap show users
```

### User Synchronization

```yaml
# When create_missing_users = true
# Creates LDAP users for policy members that don't exist

# policy.yaml
groups:
  admins: [alice, newuser]  # newuser doesn't exist in LDAP

# Plugin behavior:
# 1. Check if 'newuser' exists in LDAP
# 2. If not, create minimal user entry
# 3. Add to appa-admins group
```

## Error Handling

### Connection Issues

```bash
# Test and diagnose connection problems
appa ldap doctor

# Show connection status
appa ldap status connection

# Retry failed operations
appa ldap retry
```

### Sync Conflicts

```bash
# Handle group conflicts
appa ldap resolve conflicts

# Show conflicting objects
appa ldap show conflicts

# Manual conflict resolution
appa ldap fix conflict appa-admins
```

### Backup and Recovery

```bash
# Backup current LDAP state
appa ldap backup --output=ldap-backup.ldif

# Show what would be restored
appa ldap restore --dry-run --input=ldap-backup.ldif

# Restore from backup
appa ldap restore --input=ldap-backup.ldif
```

## Security Considerations

### Authentication

- **Dedicated Service Account**: Use dedicated LDAP service account for appa
- **Minimal Permissions**: Grant only necessary LDAP permissions
- **Kerberos Integration**: Use keytabs for FreeIPA authentication
- **Encrypted Connections**: Always use LDAPS or StartTLS

### Authorization

```yaml
# Recommended LDAP permissions for appa service account:
permissions:
  - "cn=groups,cn=accounts,dc=homelab,dc=local": ["read", "write", "add", "delete"]
  - "cn=hostgroups,cn=hostgroups,cn=accounts,dc=homelab,dc=local": ["read", "write", "add"]
  - "cn=hbac,dc=homelab,dc=local": ["read", "write", "add"]
```

### Audit Trail

```bash
# Show sync history
appa ldap audit

# Show changes made by appa
appa ldap changes --since=2024-01-01

# Export audit log
appa ldap export audit --format=json
```

## Common Workflows

### Initial Setup

```bash
# Configure LDAP plugin
appa plugin configure ldap

# Test connection
appa ldap test connection

# Initial sync (dry run)
appa ldap sync --dry-run

# Perform initial sync
appa ldap sync all
```

### Regular Operations

```bash
# Daily sync automation
appa ldap sync groups --quiet

# Validate sync state
appa ldap validate

# Clean up orphaned objects
appa ldap cleanup --dry-run
```

### Policy Changes

```bash
# After updating policy.yaml
appa validate policy
appa ldap sync --dry-run
appa ldap sync groups
```

### System Addition

```bash
# After adding new system
appa system add new-web --type=server --env=prod
appa system set new-web --profile+=web-server
appa ldap sync hostgroups  # Update FreeIPA hostgroups
```

## Best Practices

### Group Naming
- Use consistent prefixes to identify appa-managed groups
- Avoid conflicts with existing LDAP group names
- Use descriptive names that map to appa concepts

### Sync Frequency
- Run group sync after policy changes
- Schedule regular validation checks
- Monitor sync failures and conflicts

### Security
- Use dedicated service accounts with minimal permissions
- Regularly audit group memberships
- Monitor LDAP access logs for appa operations

### Integration
- Coordinate with existing LDAP group structure
- Document group mappings for other administrators
- Test access control after sync operations

## Troubleshooting

### Common Issues

```bash
# Cannot connect to LDAP server
appa ldap doctor --verbose

# Groups not syncing
appa ldap sync groups --debug

# Permission denied
appa ldap test permissions

# Orphaned groups
appa ldap show orphaned
```

### Debug Mode

```bash
# Enable verbose logging
appa ldap sync --debug --verbose

# Show LDAP operations
appa ldap sync --trace-ldap

# Dry run with detailed output
appa ldap sync --dry-run --verbose
```

## Integration Examples

### SSH Access Control

```yaml
# policy.yaml ACL
acls:
  - action: allow
    src: [group:admins]
    dst: [tag:server:22]

# Results in FreeIPA HBAC rule:
# Rule: appa-admin-ssh-access
# Users: members of appa-admins
# Hosts: members of appa-server hostgroup
# Services: sshd
```

### Web Application Integration

```yaml
# Applications can query LDAP for group membership
# Example: Check if user is in appa-prod-access group
ldap_query: "(memberOf=cn=appa-prod-access,cn=groups,cn=accounts,dc=homelab,dc=local)"
```

### Monitoring Integration

```bash
# Monitor group membership changes
appa ldap monitor --webhook=https://monitoring.homelab.local/ldap-changes

# Alert on sync failures
appa ldap status --nagios-output
```