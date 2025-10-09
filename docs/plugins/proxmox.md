# Proxmox Plugin

The Proxmox plugin manages Proxmox VE hypervisor hosts and virtual machine lifecycle operations.

## Overview

- **Purpose**: Proxmox VE cluster and VM management
- **API**: Proxmox VE REST API
- **Features**: Host configuration, VM lifecycle, storage management
- **Integration**: Works with BMC plugin for host provisioning

## Configuration

### Plugin Settings (.appa/plugins.toml)
```toml
[plugins.proxmox]
enabled = true
default_api_port = 8006
ssl_verify = false  # For self-signed certs in homelab
api_timeout = 30
```

### Host Configuration
```yaml
# In systems/iroh.yaml (Proxmox host)
additional_tags: [tag:proxmox-host]

proxmox:
  host:
    cluster_name: "avatar-cluster"
    datacenter: "avatar-dc"
    api_port: 8006
    auth:
      realm: "pam"
      username: "root"
      token_secret: "secrets://proxmox/api_token"
    storage:
      - name: "local"
        type: "dir"
        path: "/var/lib/vz"
      - name: "local-lvm"
        type: "lvmthin"
        vgname: "pve"
        thinpool: "data"
```

### VM Configuration
```yaml
# In systems/kyoshi.yaml (VM)
proxmox:
  vm:
    vm_id: 101
    template: "nixos-template"
    node: "iroh"
    cpu: 2
    memory: 2048
    disk: 20
    network:
      bridge: "vmbr0"
      model: "virtio"
```

## Features

### Host Management
- **Cluster Setup**: Initialize and join Proxmox clusters
- **Storage Configuration**: Configure local and shared storage
- **Network Setup**: Configure bridges and VLANs
- **User Management**: Setup authentication and permissions

### VM Lifecycle
- **Creation**: Create VMs from templates or ISO
- **Configuration**: CPU, memory, disk, network settings
- **Power Management**: Start, stop, restart, suspend VMs
- **Migration**: Live migrate VMs between hosts
- **Backup**: Schedule and manage VM backups

### Template Management
- **Creation**: Create templates from existing VMs
- **Updates**: Update base templates with latest packages
- **Deployment**: Deploy VMs from templates
- **Customization**: Cloud-init integration for VM customization

## CLI Commands

### Host Operations
```bash
# Host management
appa iroh show proxmox             # Show cluster status
appa iroh proxmox status           # Node status and resources
appa iroh proxmox storage          # Storage configuration
appa iroh proxmox network          # Network configuration

# Cluster operations
appa iroh proxmox join-cluster --cluster=avatar-cluster
appa iroh proxmox leave-cluster
```

### VM Operations
```bash
# VM lifecycle
appa kyoshi start                  # Start VM
appa kyoshi stop                   # Stop VM
appa kyoshi restart                # Restart VM
appa kyoshi suspend                # Suspend VM

# VM management
appa kyoshi show vm                # VM configuration and status
appa kyoshi resize --cpu=4 --memory=4096
appa kyoshi migrate --target=iroh2
appa kyoshi backup --storage=backup-storage
```

### Template Operations
```bash
# Template management
appa template list
appa template show nixos-template
appa template create --from=kyoshi --name=nginx-template
appa template update nixos-template
```

## Integration Points

### BMC Plugin Integration
1. **BMC Plugin**: Provisions physical hardware with Proxmox VE
2. **Proxmox Plugin**: Configures cluster, storage, and networking
3. **Other Backends**: Configure VMs using Nix/Ansible/Docker

### VM Creation Workflow
1. **Template Preparation**: Ensure VM template exists
2. **VM Creation**: Create VM from template with specified resources
3. **Network Setup**: Configure network bridges and VLANs
4. **Backend Handoff**: VM ready for configuration management

## Storage Types

### Local Storage
```yaml
storage:
  - name: "local"
    type: "dir"
    path: "/var/lib/vz"
    content: ["iso", "vztmpl"]
```

### LVM Storage
```yaml
storage:
  - name: "local-lvm"
    type: "lvmthin"
    vgname: "pve"
    thinpool: "data"
    content: ["images", "rootdir"]
```

### Shared Storage
```yaml
storage:
  - name: "nfs-storage"
    type: "nfs"
    server: "storage.avatar"
    export: "/mnt/proxmox"
    content: ["images", "iso", "backup"]
```

## Security

### Authentication
- **API Tokens**: Preferred for automation
- **User Accounts**: Traditional username/password
- **Realm Integration**: LDAP/AD authentication

### Network Security
- **Management Network**: Separate network for cluster communication
- **Firewall Rules**: Built-in Proxmox firewall
- **SSL/TLS**: HTTPS API access

### Backup and Recovery
- **Scheduled Backups**: Automated VM backups
- **Backup Encryption**: Encrypted backup storage
- **Disaster Recovery**: Cluster backup and restore procedures

## Examples

### Complete Host Configuration
```yaml
name: iroh
description: "Proxmox VE hypervisor node"
type: hypervisor
additional_tags: [tag:proxmox-host, tag:bare-metal]

proxmox:
  host:
    cluster_name: "avatar-cluster"
    datacenter: "avatar-dc"
    storage:
      - name: "local-lvm"
        type: "lvmthin"
        vgname: "pve"
        thinpool: "data"
    network:
      bridges:
        - name: "vmbr0"
          interface: "ens18"
          cidr: "10.0.1.0/24"
```

### Complete VM Configuration
```yaml
name: kyoshi
description: "Nginx reverse proxy server"
type: server

proxmox:
  vm:
    vm_id: 101
    template: "nixos-template"
    node: "iroh"
    resources:
      cpu: 2
      memory: 2048
      disk: 20
    network:
      - bridge: "vmbr0"
        model: "virtio"
        firewall: true
```

See the [sample homelab](../../sample/homelab/systems/) for complete Proxmox configuration examples.