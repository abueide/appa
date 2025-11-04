# Secret Commands

Secret operations manage SOPS-encrypted sensitive data that can be referenced throughout your homelab configuration using the `secrets://` URI scheme.

## Secret Operations

### View Secrets

```bash
# List all secret keys (encrypted values not shown)
appa secret show
appa secret show --keys-only

# Get specific secret value (decrypted)
appa secret get ldap/bind_password
appa secret get ssl/private_key

# Show secret metadata
appa secret info gitlab/root_password
```

### Manage Secrets

```bash
# Set new secret value (prompts for value)
appa secret set ldap/bind_password

# Set secret from stdin
echo "secret_value" | appa secret set api/token

# Set secret from file
appa secret set ssl/cert --from-file=/path/to/cert.pem

# Generate random secret
appa secret generate db/password --length=32
appa secret generate api/key --type=hex --length=64

# Remove secret
appa secret remove database.password
appa secret remove api.key --force
```

### Edit Secrets

```bash
# Open secrets.yaml in $EDITOR (decrypted)
appa secret edit

# Edit specific secret interactively
appa secret edit-value gitlab/root_password
```

### Validation

```bash
# Validate secrets file integrity
appa secret validate
appa secret validate --check=encryption
appa secret validate --check=references

# Check for unused secrets
appa secret check-usage

# Find secrets referenced but not defined
appa secret check-references

# Test decryption with all available keys
appa secret test-decrypt
```

### Key Management

```bash
# Re-encrypt with current keys
appa secret encrypt

# Decrypt for manual editing (dangerous)
appa secret decrypt

# Rotate encryption keys
appa secret rotate-keys

# Add new encryption key
appa secret add-key --age-key="age1..."
appa secret add-key --gpg-key="fingerprint"

# Remove encryption key
appa secret remove-key --age-key="age1..."
```

### Secret Rotation

```bash
# Generate new value for existing secret
appa secret rotate gitlab/root_password

# Rotate multiple secrets
appa secret rotate db/password api/token ssl/key

# Rotate all secrets matching pattern
appa secret rotate "db/*"
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
appa secret init --age

# GPG encryption for team environments
appa secret init --gpg --key="your@email.com"

# Multiple keys for redundancy
appa secret add-key --age-key="age1..."
appa secret add-key --gpg-key="fingerprint"
```

### Access Control

```bash
# Check who can decrypt secrets
appa secret list-keys

# Audit secret access
appa secret audit-log

# Rotate keys (re-encrypt with new keys)
appa secret rotate-keys --remove-old
```

### Backup and Recovery

```bash
# Export encrypted secrets (safe to backup)
appa secret export --output=backup.yaml

# Import secrets from backup
appa secret import --input=backup.yaml

# Decrypt for emergency recovery (store securely!)
appa secret decrypt --output=emergency-backup.yaml
```

## Common Flags

- `--decrypt` - Show decrypted secret value
- `--keys-only` - Show only secret keys, not values
- `--value=VALUE` - Set secret value directly
- `--from-file=PATH` - Set secret from file contents
- `--length=N` - Length for generated secrets
- `--type=TYPE` - Type for generated secrets (hex, base64, alphanumeric)
- `--usage` - Show where secret is referenced
- `--unused` - Show unreferenced secrets
- `--check=TYPE` - Validation check type (encryption, references)

## Error Handling

### Common Issues

```bash
# Cannot decrypt secrets
appa secret doctor                    # Diagnose decryption issues
appa secret test-decrypt --verbose    # Test with debug output

# Missing secret references
appa secret check-references          # Find undefined secrets
appa system validate --all --check=secrets  # Validate across all objects

# Corrupted secrets file
appa secret backup --verify           # Create verified backup
appa secret restore --from-backup     # Restore from backup
```

### Troubleshooting

```bash
# Debug secret resolution
appa secret trace gitlab/root_password

# Show encryption key status
appa secret key-status

# Validate file format
appa secret lint
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

## See Also

- [Secret Guide](../01-user-guide/secret.md) - Concepts and workflows
- [SOPS Plugin](../03-plugins/sops.md) - Encryption backend
- [Security Best Practices](../05-contributing/) - Secret handling guidelines