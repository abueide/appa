# SOPS Plugin

The SOPS plugin provides encrypted secrets management using Mozilla SOPS with Age and GPG backends.

## Overview

- **Purpose**: Encrypted secrets management
- **Backend**: Mozilla SOPS
- **Encryption**: Age keys (default) and GPG support
- **Status**: Core plugin (always loaded)

## Configuration

### Plugin Settings (.appa/plugins.toml)
```toml
[plugins.sops]
enabled = true
backend = "age"
config_file = "secrets.yaml"
age_key_file = "~/.config/sops/age/keys.txt"
```

### Secret File Structure (secrets.yaml)
```yaml
metadata:
  version: "1.0"
  backend: "sops"

sops:
  keys:
    age:
      - "age1ql3z7hjy54pw9hyww5ayyfg7zqgvc7w3j2elw8zmrj2kg5sfn9aqmcac8p"
    pgp:
      - "FBC7B9E2A4F9289AC0C1D4843D16CEE4A27381B4"

secrets:
  ldap:
    bind_password: ENC[AES256_GCM,data:...,tag:...,type:str]
```

## Features

### Secret Storage
- **Encrypted Values**: All sensitive data encrypted at rest
- **Git-Safe**: Encrypted files can be committed to version control
- **Multiple Keys**: Support for multiple Age/GPG keys
- **Public Keys**: Non-sensitive keys stored unencrypted

### Secret References
Use `secrets://` URI scheme in configurations:
```yaml
proxmox:
  host:
    auth:
      token_secret: "secrets://proxmox/api_token"
```

### Key Management
- **Age Keys**: Current encryption standard (recommended)
- **GPG Keys**: Traditional PGP encryption for teams
- **Key Rotation**: Update encryption keys without losing data
- **Per-Key Access**: Control who can decrypt which secrets

## CLI Commands

```bash
# View secrets
appa secrets show                    # List all secret paths
appa secrets get ldap/bind_password  # Get decrypted value

# Manage secrets
appa secrets set ldap/bind_password  # Set new secret value
appa secrets edit                    # Edit secrets.yaml directly
appa secrets rotate gitlab/root_password  # Generate new random value

# Key management
appa secrets encrypt                 # Re-encrypt with current keys
appa secrets decrypt                 # Decrypt for manual editing
```

## Security Model

### Encryption
- **AES-256-GCM**: Symmetric encryption for secret values
- **Age/GPG**: Asymmetric encryption for key management
- **Deterministic**: Same plaintext produces different ciphertext

### Access Control
- **Key-Based**: Access controlled by possession of private keys
- **Per-Secret**: Different secrets can use different keys
- **Team Support**: Multiple team members can have access

### Git Integration
- **Safe Commits**: Encrypted files safe to commit
- **Diff-Friendly**: Changes to individual secrets are isolated
- **History**: Full audit trail in Git history

## Installation

The SOPS plugin is a core plugin and comes pre-installed with Appa.

### Age Setup
```bash
# Generate age key
age-keygen -o ~/.config/sops/age/keys.txt

# Add public key to secrets.yaml
# Add private key path to plugins.toml
```

### GPG Setup
```bash
# Generate GPG key
gpg --gen-key

# Export public key
gpg --armor --export your@email.com

# Add fingerprint to secrets.yaml
```

## Best Practices

- **Use Age**: Fewer setup steps than GPG
- **Rotate Keys**: Regularly rotate encryption keys
- **Minimal Access**: Only grant access to necessary secrets
- **Environment Separation**: Different keys for prod/dev/staging
- **Backup Keys**: Securely backup private keys

## Examples

See the [sample homelab secrets](../../sample/homelab/secrets.yaml) for complete examples.