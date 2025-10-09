# Secrets

Secrets are encrypted configuration values managed by the SOPS plugin and stored in `secrets.yaml`.

## Schema

### Metadata
```yaml
metadata:
  version: "1.0"
  backend: "sops"
```

### SOPS Configuration
```yaml
sops:
  keys:
    # Age keys for local development
    age:
      - "age1ql3z7hjy54pw9hyww5ayyfg7zqgvc7w3j2elw8zmrj2kg5sfn9aqmcac8p"

    # GPG keys for team members
    pgp:
      - "FBC7B9E2A4F9289AC0C1D4843D16CEE4A27381B4"  # alice@home.lab
```

### Secret Definitions
```yaml
secrets:
  # LDAP service account
  ldap:
    bind_password: ENC[AES256_GCM,data:...,tag:...,type:str]

  # Proxmox API access
  proxmox:
    api_token: ENC[AES256_GCM,data:...,tag:...,type:str]
    root_password: ENC[AES256_GCM,data:...,tag:...,type:str]

  # SSH keys for systems
  ssh:
    admin_private_key: ENC[AES256_GCM,data:...,tag:...,type:str]
```

### Public Keys
```yaml
# Public keys (not encrypted)
public_keys:
  ssh:
    admin: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... admin@laptop"
    alice: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... alice@workstation"
```

## Secret References

Secrets are referenced using the `secrets://` URI scheme in system configurations:

```yaml
# In system configuration
proxmox:
  host:
    auth:
      token_secret: "secrets://proxmox/api_token"

bmc:
  post_install:
    ssh_keys:
      - "secrets://ssh/admin"
```

## Design Principles

- **Encrypted at Rest**: All sensitive values encrypted with SOPS
- **Git-Friendly**: Encrypted files can be safely committed
- **Multiple Backends**: Support for Age, GPG, and future backends
- **URI References**: Clear secret references with `secrets://` scheme
- **Public Key Storage**: Non-sensitive keys stored unencrypted

## CLI Commands

### Secret Management
```bash
appa secrets show                    # List all secret keys
appa secrets get ldap/bind_password  # Get specific secret value
appa secrets set ldap/bind_password  # Set secret (prompts for value)
appa secrets edit                    # Open secrets.yaml in $EDITOR
appa secrets encrypt                 # Re-encrypt with new keys
appa secrets decrypt                 # Decrypt for editing
appa secrets rotate                  # Generate new secret values
```

## Security Features

- **Age/GPG Encryption**: Industry-standard encryption backends
- **Key Rotation**: Support for rotating encryption keys
- **Access Control**: Per-key access control via encryption keys
- **Audit Trail**: Git history tracks secret changes
- **No Plaintext**: Secrets never stored in plaintext

## Examples

See the [sample homelab](../sample/homelab/secrets.yaml) for complete secret examples.