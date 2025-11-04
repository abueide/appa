# BMC Plugin

The BMC (Baseboard Management Controller) plugin provides bare metal server provisioning via IPMI and Redfish protocols.

## Overview

- **Purpose**: Bare metal server provisioning and management
- **Protocols**: IPMI, Redfish
- **Vendors**: Supermicro, Dell, HP, and others
- **Integration**: Installs base OS, then hands off to other backends

## Configuration

### Plugin Settings (.appa/plugins.toml)
```toml
[plugins.bmc]
enabled = true
default_vendor = "supermicro"
timeout_seconds = 300

# Vendor-specific configurations
vendors.supermicro.driver = "ipmi"
vendors.dell.driver = "redfish"
vendors.hp.driver = "redfish"
```

### System Configuration
```yaml
# In systems/iroh.yaml
hardware:
  bmc:
    ip: 10.0.0.5
    mac: "aa:bb:cc:dd:ee:ff"
    vendor: "supermicro"

bmc:
  install_image: "proxmox-ve-8.1-iso"
  boot_mode: "uefi"
  raid_config: "raid1"
  post_install:
    ssh_keys:
      - "secrets://ssh/admin"
    packages:
      - "qemu-guest-agent"
      - "curl"
```

## Features

### Hardware Management
- **Power Control**: Power on/off, reset, power cycle
- **Boot Management**: Set boot device, UEFI/BIOS settings
- **Hardware Monitoring**: Temperature, fans, power consumption
- **RAID Configuration**: Setup hardware RAID arrays

### OS Installation
- **Image Mounting**: Mount ISO images via BMC virtual media
- **Automated Install**: Unattended OS installation
- **Post-Install Setup**: Basic SSH access and package installation
- **Handoff**: Transfer control to configuration management backends

### Vendor Support
- **Supermicro**: IPMI 2.0 support
- **Dell iDRAC**: Redfish API integration
- **HP iLO**: Redfish API integration
- **Generic IPMI**: Standard IPMI 2.0 protocol

## Workflow

### Provisioning Process
1. **Discovery**: Detect BMC and hardware capabilities
2. **Preparation**: Configure RAID, set boot device
3. **Installation**: Mount ISO and trigger automated install
4. **Post-Install**: Configure SSH access and basic packages
5. **Handoff**: Mark system ready for backend configuration

### Integration with Other Backends
```yaml
# BMC installs base OS
type: hypervisor
backend: ansible  # Takes over after BMC completes

# Tags indicate BMC management
additional_tags: [tag:bare-metal, tag:proxmox-host]
```

## CLI Commands

### Hardware Management
```bash
# Power control
appa iroh power on
appa iroh power off
appa iroh power reset

# Hardware status
appa iroh show hardware            # Temperature, fans, power
appa iroh show bmc                 # BMC firmware and settings
```

### Provisioning
```bash
# Full provisioning workflow
appa iroh provision                # Complete BMC provisioning

# Manual steps
appa iroh install --image=proxmox-ve-8.1-iso
appa iroh configure --raid=raid1
appa iroh validate hardware
```

## Supported Hardware

### Supermicro
- **Protocol**: IPMI 2.0
- **Features**: Full hardware management, virtual media
- **Authentication**: Username/password, LDAP integration

### Dell PowerEdge
- **Protocol**: Redfish + iDRAC
- **Features**: Hardware management, lifecycle controller
- **Authentication**: Local users, Active Directory

### HP ProLiant
- **Protocol**: Redfish + iLO
- **Features**: Hardware management, automated provisioning
- **Authentication**: Local users, LDAP/AD integration

## Security

### Authentication
- **IPMI Credentials**: Stored in secrets.yaml
- **Certificate Validation**: For Redfish connections
- **Network Isolation**: BMC on separate management network

### Access Control
- **Role-Based**: Different privilege levels
- **Network Restrictions**: Limit BMC network access
- **Audit Logging**: Track all BMC operations

## Configuration Examples

### Supermicro System
```yaml
hardware:
  bmc:
    ip: 10.0.0.5
    vendor: "supermicro"
    username: "ADMIN"
    password: "secrets://bmc/supermicro_password"

bmc:
  install_image: "ubuntu-22.04-server.iso"
  boot_mode: "uefi"
  raid_config: "raid1"
```

### Dell System
```yaml
hardware:
  bmc:
    ip: 10.0.0.6
    vendor: "dell"
    username: "root"
    password: "secrets://bmc/dell_password"

bmc:
  install_image: "esxi-7.0-installer.iso"
  boot_mode: "uefi"
  idrac_settings:
    virtual_console: true
    virtual_media: true
```

## Examples

See the [sample homelab](../../sample/homelab/systems/iroh.yaml) for complete BMC configuration examples.