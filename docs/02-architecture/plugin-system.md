# Modules

Modules contain configuration implementations that can be applied to systems via plugin handlers. Each module is a directory containing an appa.yaml definition and implementation files for one or more backends.

## Schema

### Module Definition (appa.yaml)
```yaml
name: web-server                     # Globally unique identifier
description: "Nginx web server configuration with SSL support"
format_version: "1.0"

# Module-specific properties
type: module
handler: nix                        # Primary plugin handler
alternative_handlers:               # Optional: other supported handlers
  - ansible
  - docker

# Handler-specific configurations
handler_config:
  nix:
    file: "configuration.nix"
    type: "nixos-module"
  ansible:
    file: "playbook.yml"
    type: "playbook"
  docker:
    file: "compose.yml"
    type: "docker-compose"

# Variable definitions for templating
variables:
  hostname:
    type: string
    required: true
    description: "System hostname"
  domain:
    type: string
    required: false
    default: "homelab.local"
    description: "Domain name"
  ssl_enabled:
    type: boolean
    default: true
    description: "Enable SSL/TLS termination"

# Dependencies on other modules
dependencies:
  - name: "ssl-certificates"
    version: ">=1.0"
  - name: "base-security"
    version: "*"

# Tags for module discovery
tags:
  - "web"
  - "nginx"
  - "ssl"
```

## Module Organization

Modules are organized as directories with implementation files:
```
modules/
├── web-server/
│   ├── appa.yaml              # Module definition
│   ├── configuration.nix      # Nix implementation
│   ├── playbook.yml           # Ansible implementation
│   └── compose.yml            # Docker implementation
├── database/
│   ├── appa.yaml
│   ├── flake.nix
│   └── deployment.yaml        # Kubernetes implementation
└── monitoring/
    ├── appa.yaml
    └── docker-compose.yml
```

## Naming Convention

- **Format**: `<purpose>` (no backend suffix)
- **Examples**: `web-server`, `database`, `monitoring`
- **Global Uniqueness**: Module names must be unique
- **Handler Support**: One module can support multiple backends

## CLI Commands

### Module Management
```bash
appa modules list                    # List available modules
appa modules show web-server         # Show module definition
appa modules validate web-server     # Validate module structure
appa modules test web-server --system=web-01  # Test module against system
```

### Module Application
```bash
# Apply module to system (via profile or directly)
appa web-01 apply-module web-server --dry-run
appa web-01 deploy --validate        # Apply all assigned profiles/modules
```

## Module Content

Each module directory contains handler-specific implementations:

### Nix Implementation Example
```nix
# modules/web-server/configuration.nix
{ config, pkgs, ... }:
{
  services.nginx = {
    enable = true;
    virtualHosts."{{ hostname }}.{{ domain }}" = {
      enableACME = {{ ssl_enabled }};
      forceSSL = {{ ssl_enabled }};
      locations."/" = {
        root = "/var/www";
      };
    };
  };

  networking.hostName = "{{ hostname }}";
  networking.domain = "{{ domain }}";
}
```

### Ansible Implementation Example
```yaml
# modules/web-server/playbook.yml
---
- name: Configure Nginx Web Server
  hosts: all
  become: yes
  vars:
    hostname: "{{ hostname }}"
    domain: "{{ domain }}"
    ssl_enabled: "{{ ssl_enabled }}"

  tasks:
    - name: Install Nginx
      package:
        name: nginx
        state: present

    - name: Configure Nginx virtual host
      template:
        src: nginx.conf.j2
        dest: "/etc/nginx/sites-available/{{ hostname }}.{{ domain }}"
      notify: restart nginx

  handlers:
    - name: restart nginx
      service:
        name: nginx
        state: restarted
```

### Docker Implementation Example
```yaml
# modules/web-server/compose.yml
version: '3.8'
services:
  nginx:
    image: nginx:alpine
    hostname: {{ hostname }}
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - /var/www:/usr/share/nginx/html:ro
    environment:
      - DOMAIN={{ domain }}
      - SSL_ENABLED={{ ssl_enabled }}
```

## Plugin Handler Interface

See [Module Interface Documentation](modules/module-interface.md) for complete details on:
- Handler registration and execution protocol
- Variable substitution system
- Input/output formats
- Error handling

## Examples

See the [sample homelab](../../sample/homelab/modules/) for complete module examples.