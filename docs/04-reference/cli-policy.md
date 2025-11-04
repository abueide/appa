# Policy Commands

Policy operations manage access control rules and profile enforcement policies for your homelab infrastructure.

## Policy Operations

```bash
# Show current policies
appa policy show
appa policy show --format=json

# Validate policies
appa policy validate
appa policy validate --check=syntax
appa policy validate --check=references

# Edit policy file
appa policy edit

# Test policy rules
appa policy test --user=alice --system=web-01
appa policy test --rule=ssh-access --dry-run

# Apply policy changes
appa policy apply --validate
appa policy apply --dry-run

# Show policy violations
appa policy check --violations
appa policy check --user=alice
appa policy check --system=web-01
```

## Policy Structure

Policies are defined in a single `policy.yaml` file containing:

- **ACL Rules** - Access control definitions
- **Profile Enforcement** - Required profiles for system types
- **User Groups** - LDAP group mappings
- **Network Policies** - Tailscale-inspired network access

## Policy Types

### Access Control Lists (ACLs)
- **ssh-access** - SSH connectivity rules
- **admin-access** - Administrative permissions
- **deploy-access** - Deployment permissions

### Profile Enforcement
- **required-profiles** - Mandatory profiles per system type
- **environment-policies** - Environment-specific requirements

### Network Policies
- **mesh-access** - Direct connectivity rules
- **port-access** - Service-specific access

## Common Flags

- `--check=TYPE` - Validation check type (syntax, references, violations)
- `--user=USER` - Test policies for specific user
- `--system=SYSTEM` - Test policies for specific system
- `--rule=RULE` - Test specific policy rule
- `--violations` - Show current policy violations
- `--validate` - Validate before applying changes

## Policy Validation

```bash
# Check policy syntax
appa policy validate --check=syntax

# Check cross-references
appa policy validate --check=references

# Check for violations
appa policy check --violations

# Test user access
appa policy test --user=alice --system=web-01
```

## See Also

- [Policy Guide](../01-user-guide/policy.md) - Detailed policy management
- [System Commands](cli-system.md) - System management
- [LDAP Integration](../03-plugins/ldap.md) - User group management