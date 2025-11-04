# Ansible Plugin

The Ansible plugin provides configuration management through playbooks and roles for systems that don't use NixOS.

## Overview

- **Purpose**: Configuration management for Ubuntu, CentOS, and other Linux distributions
- **Features**: Playbooks, roles, inventory management, vault integration
- **Integration**: Works with existing Ansible infrastructure
- **Status**: Optional plugin for non-NixOS systems

## Configuration

### Plugin Settings (.appa/plugins.toml)
```toml
[plugins.ansible]
enabled = true
inventory_format = "yaml"
vault_password_file = "~/.ansible/vault_pass"
host_key_checking = false
```

### System Configuration
```yaml
# In systems/toph.yaml
backend: ansible
backend_config:
  ansible_host: "toph.avatar"
  ansible_user: "ubuntu"
  ansible_ssh_private_key_file: "secrets://ssh/admin_private_key"
```

## Features

### Playbook Management
- **Role-Based**: Organize configuration into reusable roles
- **Task Execution**: Idempotent task execution
- **Handler System**: Restart services when configs change
- **Conditional Logic**: Apply configs based on system facts

### Inventory Integration
- **Dynamic Inventory**: Generate Ansible inventory from Appa systems
- **Group Management**: Automatic host groups based on tags
- **Variable Management**: Pass system properties as Ansible variables
- **Vault Integration**: Secure secrets with Ansible Vault

### Multi-Distribution Support
- **Ubuntu/Debian**: APT package management
- **CentOS/RHEL**: YUM/DNF package management
- **Generic Linux**: Shell commands and file management
- **Container Hosts**: Docker and Podman configuration

## Module Structure

### Playbook Definition
```yaml
# modules/ansible/web-server.yml
---
- name: Configure web server
  hosts: "{{ target_host }}"
  become: yes

  vars:
    domain: "{{ webserver_domain | default('localhost') }}"
    ssl_enabled: "{{ webserver_ssl | default(false) }}"

  tasks:
    - name: Install Nginx
      package:
        name: nginx
        state: present

    - name: Configure Nginx virtual host
      template:
        src: nginx.conf.j2
        dest: "/etc/nginx/sites-available/{{ domain }}"
        backup: yes
      notify: restart nginx

    - name: Enable virtual host
      file:
        src: "/etc/nginx/sites-available/{{ domain }}"
        dest: "/etc/nginx/sites-enabled/{{ domain }}"
        state: link
      notify: restart nginx

    - name: Start and enable Nginx
      service:
        name: nginx
        state: started
        enabled: yes

  handlers:
    - name: restart nginx
      service:
        name: nginx
        state: restarted
```

### Role Structure
```
modules/ansible/roles/docker-host/
├── tasks/
│   └── main.yml
├── handlers/
│   └── main.yml
├── templates/
│   └── docker-daemon.json.j2
├── files/
│   └── docker-compose.yml
└── vars/
    └── main.yml
```

### Module Registration
```yaml
# In profiles/docker-host.yaml
configurations:
  - backend: ansible
    module: "docker-host-ansible"
    variables:
      docker_version: "20.10"
      compose_version: "2.0"
```

## CLI Commands

### Playbook Execution
```bash
# Run specific playbook
appa toph ansible run               # Run all assigned playbooks
appa toph ansible run web-server-ansible  # Run specific module
appa toph ansible check             # Dry run (check mode)
appa toph ansible diff              # Show what would change
```

### Inventory Management
```bash
# Inventory operations
appa ansible inventory             # Show generated inventory
appa ansible groups               # Show host groups
appa ansible vars toph            # Show variables for host
```

### Vault Operations
```bash
# Secrets management
appa ansible vault edit           # Edit vault file
appa ansible vault encrypt        # Encrypt file
appa ansible vault decrypt        # Decrypt file
```

## Integration Points

### With Proxmox Plugin
1. **VM Creation**: Proxmox creates VM with base OS
2. **Initial Setup**: Basic SSH access configured
3. **Ansible Plugin**: Applies full system configuration
4. **Updates**: Ansible manages packages and services

### With Secrets Plugin
```yaml
# Reference secrets in playbook
- name: Set database password
  mysql_user:
    name: myapp
    password: "{{ vault_database_password }}"
```

## Generated Inventory

Appa automatically generates Ansible inventory from system definitions:

```yaml
# Generated inventory.yml
all:
  children:
    servers:
      hosts:
        toph.avatar:
          ansible_host: 10.0.1.30
          ansible_user: ubuntu
          system_type: server
          system_env: prod

    docker_hosts:
      hosts:
        toph.avatar:
          docker_version: "20.10"

    production:
      hosts:
        toph.avatar:
          monitoring_enabled: true
```

### Group Assignment
Systems are automatically assigned to groups based on:
- **Type**: `servers`, `workstations`, `devices`
- **Environment**: `production`, `development`, `staging`
- **Tags**: Custom groups from system tags
- **Profiles**: Groups based on assigned profiles

## Configuration Patterns

### Package Management
```yaml
- name: Install packages
  package:
    name: "{{ item }}"
    state: present
  loop:
    - htop
    - curl
    - git
```

### Service Configuration
```yaml
- name: Configure service
  template:
    src: myservice.conf.j2
    dest: /etc/myservice/myservice.conf
  notify: restart myservice

- name: Start service
  service:
    name: myservice
    state: started
    enabled: yes
```

### File Management
```yaml
- name: Create directory
  file:
    path: /opt/myapp
    state: directory
    owner: myapp
    group: myapp
    mode: '0755'

- name: Copy configuration
  copy:
    src: myapp.conf
    dest: /opt/myapp/myapp.conf
    backup: yes
```

## Best Practices

### Playbook Organization
- **Role-Based**: Use roles for complex configurations
- **Idempotent**: Ensure tasks can run multiple times safely
- **Error Handling**: Use `failed_when` and `ignore_errors` appropriately

### Variable Management
- **Group Variables**: Common settings in group_vars
- **Host Variables**: Host-specific settings in host_vars
- **Vault Encryption**: Encrypt sensitive variables

### Testing
- **Check Mode**: Always test with `--check` first
- **Staging**: Test on development systems first
- **Rollback**: Plan rollback procedures for changes

## Examples

See the [sample homelab modules](../../sample/homelab/modules/ansible/) for complete Ansible configuration examples.