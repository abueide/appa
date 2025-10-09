# Secrets Subcommand

The `appa secrets` subcommand manages encrypted secrets using the SOPS plugin by default.

## Overview

Secrets are stored in an encrypted `secrets.yaml` file and referenced in configurations using the `secrets://` URI scheme. The secrets subcommand provides secure management of sensitive data like passwords, API tokens, and private keys.

## Commands

### View Secrets

```bash
# List all secret keys (encrypted values not shown)
appa secrets show

# Get specific secret value (decrypted)
appa secrets get ldap/bind_password
appa secrets get ssl/private_key

# Show secret metadata
appa secrets info gitlab/root_password
```

### Manage Secrets

```bash
# Set new secret value (prompts for value)
appa secrets set ldap/bind_password

# Set secret from stdin
echo "secret_value" | appa secrets set api/token

# Set secret from file
appa secrets set ssl/cert --from-file=/path/to/cert.pem

# Generate random secret
appa secrets generate db/password --length=32
appa secrets generate api/key --type=hex --length=64
```

### Edit Secrets

```bash
# Open secrets.yaml in $EDITOR (decrypted)
appa secrets edit

# Edit specific secret interactively
appa secrets edit-value gitlab/root_password
```

### Key Management

```bash
# Re-encrypt with current keys
appa secrets encrypt

# Decrypt for manual editing (dangerous)
appa secrets decrypt

# Rotate encryption keys
appa secrets rotate-keys

# Add new encryption key
appa secrets add-key --age-key="age1..."
appa secrets add-key --gpg-key="fingerprint"

# Remove encryption key
appa secrets remove-key --age-key="age1..."
```

### Secret Rotation

```bash
# Generate new value for existing secret
appa secrets rotate gitlab/root_password

# Rotate multiple secrets
appa secrets rotate db/password api/token ssl/key

# Rotate all secrets matching pattern
appa secrets rotate "db/*"
```

### Validation

```bash
# Validate secrets file integrity
appa secrets validate

# Check for unused secrets
appa secrets check-usage

# Find secrets referenced but not defined
appa secrets check-references

# Test decryption with all available keys
appa secrets test-decrypt
```

## Secret URI Scheme

Secrets are referenced in configurations using the `secrets://` URI scheme:

```yaml
# In system configurations
proxmox:
  host:
    auth:
      token_secret: "secrets://proxmox/api_token"

# In profile configurations
database:
  root_password: "secrets://db/root_password"
  ssl_cert: "secrets://ssl/wildcard_cert"
  ssl_key: "secrets://ssl/wildcard_key"

# SSH keys for BMC
bmc:
  post_install:
    ssh_keys:
      - "secrets://ssh/admin_key"
```

## Secret Organization

### Recommended Structure

```yaml
secrets:
  # Authentication credentials
  ldap:
    bind_password: "..."
    admin_password: "..."

  # Service API tokens
  proxmox:
    api_token: "..."
    root_password: "..."

  # SSH keys
  ssh:
    admin_key: "..."           # Public key
    admin_private_key: "..."   # Private key
    host_keys:
      iroh_host_key: "..."

  # Database credentials
  db:
    root_password: "..."
    backup_password: "..."

  # Application secrets
  gitlab:
    root_password: "..."
    secret_key_base: "..."

  # SSL certificates
  ssl:
    wildcard_cert: "..."
    wildcard_key: "..."
    ca_cert: "..."
```

### Naming Conventions

- **Hierarchical**: Use forward slashes for grouping: `service/secret_name`
- **Descriptive**: Clear purpose: `gitlab/root_password`, `ssl/wildcard_cert`
- **Consistent**: Follow patterns: `*_password`, `*_key`, `*_token`
- **No Spaces**: Use underscores or hyphens: `secret_key_base`, `api-token`

## Security Features

### Encryption

```bash
# Default: Age encryption (recommended)
appa secrets init --age

# GPG encryption for team environments
appa secrets init --gpg --key="your@email.com"

# Multiple keys for redundancy
appa secrets add-key --age-key="age1..."
appa secrets add-key --gpg-key="fingerprint"
```

### Access Control

```bash
# Check who can decrypt secrets
appa secrets list-keys

# Audit secret access
appa secrets audit-log

# Rotate keys (re-encrypt with new keys)
appa secrets rotate-keys --remove-old
```

### Backup and Recovery

```bash
# Export encrypted secrets (safe to backup)
appa secrets export --output=backup.yaml

# Import secrets from backup
appa secrets import --input=backup.yaml

# Decrypt for emergency recovery (store securely!)
appa secrets decrypt --output=emergency-backup.yaml
```

## Integration Examples

### System Configuration

```yaml
# In systems/web-01.yaml
name: web-01
handler: nix

# Reference secrets in handler config
handler_config:
  ssl_cert: "secrets://ssl/web_cert"
  ssl_key: "secrets://ssl/web_key"

# Reference in service definitions
services:
  - name: nginx
    config:
      ssl_certificate: "secrets://ssl/web_cert"
```

### Plugin Configuration

```yaml
# In .appa/plugins.toml
[plugins.proxmox]
enabled = true
api_url = "https://proxmox.homelab.local:8006"
api_token = "secrets://proxmox/api_token"

[plugins.sops]
enabled = true
age_key_file = "secrets://sops/age_key"
```

### Profile Templates

```yaml
# In profiles/database.yaml
configurations:
  - handler: nix
    module: "postgresql"
    variables:
      root_password: "secrets://db/root_password"
      ssl_cert: "secrets://ssl/db_cert"
```

## Common Workflows

### Initial Setup

```bash
# Initialize secrets management
appa secrets init --age

# Add basic secrets
appa secrets set ldap/bind_password
appa secrets set ssh/admin_key --from-file=~/.ssh/id_rsa.pub
appa secrets set ssl/ca_cert --from-file=/etc/ssl/ca.pem
```

### Adding New Service

```bash
# Add service-specific secrets
appa secrets set gitlab/root_password
appa secrets set gitlab/secret_key_base
appa secrets set gitlab/db_password

# Validate references work
appa secrets check-references
```

### Key Rotation

```bash
# Generate new encryption key
age-keygen > new-key.txt

# Add to secrets
appa secrets add-key --age-key="$(cat new-key.txt)"

# Re-encrypt all secrets
appa secrets encrypt

# Remove old key (after confirming access)
appa secrets remove-key --age-key="old_key"
```

## Error Handling

### Common Issues

```bash
# Cannot decrypt secrets
appa secrets doctor                    # Diagnose decryption issues
appa secrets test-decrypt --verbose    # Test with debug output

# Missing secret references
appa secrets check-references          # Find undefined secrets
appa validate all --check-secrets      # Validate across all objects

# Corrupted secrets file
appa secrets backup --verify           # Create verified backup
appa secrets restore --from-backup     # Restore from backup
```

### Troubleshooting

```bash
# Debug secret resolution
appa secrets trace gitlab/root_password

# Show encryption key status
appa secrets key-status

# Validate file format
appa secrets lint
```

## Best Practices

### Security
- Use Age encryption for simplicity, GPG for teams
- Regularly rotate encryption keys and secret values
- Never commit plaintext secrets to version control
- Use separate keys for different environments (prod/dev)

### Organization
- Group related secrets hierarchically
- Use descriptive, consistent naming
- Document secret purposes in comments
- Remove unused secrets regularly

### Operations
- Automate secret rotation where possible
- Monitor secret access and usage
- Maintain emergency access procedures
- Test backup and recovery processes