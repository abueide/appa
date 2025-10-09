# PXE Plugin

The PXE plugin provides network boot capabilities for both bare metal systems and VMs, enabling automated OS installation and initial system bootstrapping.

## Overview

- **Purpose**: Network boot and OS installation via PXE/iPXE
- **Scope**: Bare metal servers and virtual machines
- **Integration**: Works with BMC plugin for complete bare metal provisioning
- **Handoff**: Bootstraps systems to a state where config management plugins can take over
- **Status**: Core plugin for infrastructure provisioning

## Configuration

### Plugin Settings (.appa/plugins.toml)
```toml
[plugins.pxe]
enabled = true
tftp_server = "10.0.1.5"
http_server = "http://10.0.1.5:8080"
default_boot_timeout = 30
dhcp_integration = true

# Boot image repositories
[plugins.pxe.images]
nixos_minimal = "https://channels.nixos.org/nixos-23.11/latest-nixos-minimal-x86_64-linux.iso"
ubuntu_server = "https://releases.ubuntu.com/22.04/ubuntu-22.04.3-live-server-amd64.iso"
proxmox_ve = "https://enterprise.proxmox.com/iso/proxmox-ve_8.1-1.iso"
alpine_virt = "https://dl-cdn.alpinelinux.org/alpine/v3.18/releases/x86_64/alpine-virt-3.18.4-x86_64.iso"
```

### PXE Server Configuration
```yaml
# PXE server system definition
name: pxe-server
type: server
env: mgmt
backend: nix

pxe:
  server:
    tftp_root: "/srv/tftp"
    http_root: "/srv/http"
    dhcp_range: "10.0.1.100-10.0.1.200"

  # Network boot menu
  menu:
    timeout: 30
    default: "local"
    entries:
      - label: "local"
        description: "Boot from local disk"
        action: "localboot"
      - label: "nixos-install"
        description: "Install NixOS"
        kernel: "nixos/kernel"
        initrd: "nixos/initrd"
        append: "console=tty0 console=ttyS0,115200n8"
```

## Features

### Network Boot Support
- **PXE/iPXE**: Traditional and enhanced network boot protocols
- **UEFI/BIOS**: Support for both boot modes
- **Multi-Architecture**: x86_64, ARM64 support
- **Serial Console**: Headless server support

### OS Installation
- **Unattended Installation**: Automated OS deployment with preseed/kickstart
- **Multiple Distributions**: NixOS, Ubuntu, Alpine, Proxmox VE, etc.
- **Custom Images**: Support for custom installation images
- **Partition Templates**: Standardized disk layout configurations

### Integration Points
- **BMC Plugin**: Power control and boot order management
- **DHCP Integration**: MAC-based boot configuration assignment
- **Post-Install Hooks**: Prepare systems for config management handoff
- **Secrets Integration**: SSH keys and initial passwords via SOPS

## System Configuration

### PXE Boot Configuration
```yaml
# In systems/server-01.yaml
name: server-01
type: server
env: prod
backend: nix

# Hardware info for BMC plugin
hardware:
  bmc:
    ip: 10.0.0.10
    mac: "aa:bb:cc:dd:ee:ff"

# PXE boot configuration
pxe:
  install:
    image: "nixos_minimal"
    boot_mode: "uefi"
    console: "ttyS0,115200n8"

    # Automated installation config
    preseed:
      hostname: "server-01"
      domain: "homelab.local"
      timezone: "UTC"
      locale: "en_US.UTF-8"

      # Partitioning scheme
      partitions:
        - device: "/dev/sda"
          scheme: "gpt"
          partitions:
            - name: "boot"
              size: "512MB"
              type: "fat32"
              mount: "/boot"
            - name: "root"
              size: "remaining"
              type: "ext4"
              mount: "/"

      # Initial user setup
      users:
        - name: "admin"
          ssh_keys:
            - "secrets://ssh/admin_key"
          sudo: true

      # Post-install preparation
      post_install:
        packages: ["curl", "git", "ssh"]
        services: ["sshd", "ntp"]

        # Prepare for config management handoff
        bootstrap:
          - "curl -sSL https://install.determinate.systems/nix | sh"
          - "systemctl enable ssh"
```

### VM PXE Configuration
```yaml
# Proxmox VM with PXE boot
proxmox:
  vm:
    vm_id: 201
    template: "pxe-boot"  # Template configured for network boot
    node: "hypervisor-01"

    # Boot configuration
    boot_order: ["net0", "scsi0"]
    bios: "ovmf"  # UEFI for modern PXE

pxe:
  install:
    image: "nixos_minimal"
    # VM-specific configuration
    vm_config:
      memory: "2GB"
      vcpus: 2
      disk_size: "20GB"
```

## Boot Images and Templates

### Image Management
```yaml
# Boot image definitions
images:
  nixos_minimal:
    url: "https://channels.nixos.org/nixos-23.11/latest-nixos-minimal-x86_64-linux.iso"
    checksum: "sha256:abcd1234..."
    kernel_path: "boot/bzImage"
    initrd_path: "boot/initrd"

  ubuntu_server:
    url: "https://releases.ubuntu.com/22.04/ubuntu-22.04.3-live-server-amd64.iso"
    checksum: "sha256:efgh5678..."
    kernel_path: "casper/vmlinuz"
    initrd_path: "casper/initrd"

  proxmox_ve:
    url: "https://enterprise.proxmox.com/iso/proxmox-ve_8.1-1.iso"
    checksum: "sha256:ijkl9012..."
    # Custom preseed for Proxmox installation
    preseed_template: "proxmox-preseed.cfg"
```

### Installation Templates
```yaml
# Preseed/kickstart templates
templates:
  nixos_server:
    base_image: "nixos_minimal"
    config: |
      { config, pkgs, ... }: {
        # Minimal NixOS configuration for bootstrap
        boot.loader.grub.enable = true;
        boot.loader.grub.device = "/dev/sda";

        networking.hostName = "{{ hostname }}";
        networking.domain = "{{ domain }}";

        users.users.admin = {
          isNormalUser = true;
          extraGroups = [ "wheel" ];
          openssh.authorizedKeys.keys = [
            "{{ ssh_public_key }}"
          ];
        };

        services.openssh.enable = true;
        security.sudo.wheelNeedsPassword = false;
      }

  ubuntu_server:
    base_image: "ubuntu_server"
    preseed: |
      d-i debian-installer/locale string en_US.UTF-8
      d-i keyboard-configuration/xkb-keymap select us
      d-i netcfg/choose_interface select auto
      d-i netcfg/get_hostname string {{ hostname }}
      d-i netcfg/get_domain string {{ domain }}

      # Partitioning
      d-i partman-auto/method string regular
      d-i partman-auto/choose_recipe select atomic
      d-i partman/confirm boolean true

      # User setup
      d-i passwd/user-fullname string Administrator
      d-i passwd/username string admin
      d-i passwd/user-password-crypted password {{ password_hash }}
```

## CLI Commands

### PXE Server Management
```bash
# PXE server operations
appa plugin configure pxe            # Configure PXE server settings
appa pxe server start                # Start PXE services
appa pxe server stop                 # Stop PXE services
appa pxe server status               # Show PXE server status

# Image management
appa pxe images list                 # List available boot images
appa pxe images download nixos_minimal  # Download boot image
appa pxe images update               # Update all images
appa pxe images verify               # Verify image checksums
```

### System Provisioning
```bash
# PXE boot operations
appa server-01 pxe boot              # Initiate PXE boot
appa server-01 pxe install nixos_minimal  # Install specific image
appa server-01 pxe status            # Show installation status

# Combined with BMC for full provisioning
appa server-01 provision            # Full BMC + PXE provisioning
appa server-01 provision --image=ubuntu_server  # With specific image
```

### Template Management
```bash
# Installation template management
appa pxe templates list              # List available templates
appa pxe templates show nixos_server # Show template details
appa pxe templates edit ubuntu_server # Edit template
appa pxe templates validate          # Validate all templates
```

## Workflow Integration

### Complete Bare Metal Provisioning
```bash
# 1. BMC plugin powers on and sets boot order
appa server-01 bmc power on --boot=pxe

# 2. PXE plugin serves boot image and installs OS
appa server-01 pxe install nixos_minimal

# 3. System boots to minimal OS with SSH access
# 4. Configuration management takes over
appa server-01 deploy --backend=nix
```

### VM Provisioning Workflow
```bash
# 1. Proxmox plugin creates VM with PXE boot
appa vm-web-01 proxmox create --boot=pxe

# 2. PXE plugin installs OS
appa vm-web-01 pxe install ubuntu_server

# 3. Configuration management deploys services
appa vm-web-01 deploy --backend=ansible
```

## Integration with Other Plugins

### BMC Plugin Integration
- **Power Management**: BMC controls power, PXE handles boot
- **Boot Order**: BMC sets network boot priority
- **Hardware Info**: Shared hardware configuration
- **Serial Console**: BMC provides console access during PXE boot

### Configuration Management Handoff
- **SSH Access**: PXE ensures SSH is configured and accessible
- **Base Packages**: Installs required tools (Nix, Docker, Python, etc.)
- **User Setup**: Creates admin users with SSH keys
- **Network Configuration**: Sets up networking for config management

### Secrets Integration
- **SSH Keys**: SOPS-encrypted keys deployed during installation
- **Passwords**: Secure password hashes for initial accounts
- **Certificates**: Optional certificate deployment for HTTPS

## Security Considerations

### Network Security
- **DHCP Security**: MAC address validation for boot assignments
- **TFTP Access**: Restrict TFTP access to management network
- **HTTP Security**: Secure boot image downloads
- **Console Access**: Serial console security

### Installation Security
- **Image Verification**: Cryptographic verification of boot images
- **Secure Boot**: UEFI Secure Boot support where applicable
- **SSH Key Management**: Secure key deployment
- **Network Isolation**: Installation network separation

## Best Practices

### PXE Server Setup
- **Dedicated Server**: Use dedicated PXE server for reliability
- **Network Placement**: Place on management network
- **Backup Images**: Maintain local copies of boot images
- **High Availability**: Consider PXE server redundancy

### Installation Templates
- **Minimal Images**: Use minimal OS installations
- **Standardized Partitioning**: Consistent disk layouts
- **SSH-First**: Always enable SSH access
- **Config Management Ready**: Install required tools and users

### Troubleshooting
- **Serial Console**: Always configure serial console access
- **Network Debugging**: DHCP and TFTP logging
- **Installation Logs**: Capture and store installation logs
- **Rollback Plan**: Plan for installation failures

## Examples

See the [sample homelab](../../sample/homelab/) for complete PXE configuration examples including:
- PXE server setup with multiple boot images
- Bare metal server PXE installation templates
- VM PXE boot configurations
- Integration with BMC and configuration management plugins