# Module Commands

Module operations manage backend-agnostic configuration abstractions that define options and interfaces for services, similar to NixOS modules but working across any deployment backend.

## Module Operations

```bash
# Create new module
appa module add nginx --description="Nginx web server module"

# List modules
appa module list
appa module list --tag=web

# Show module information
appa module show nginx
appa module show nginx --options
appa module show nginx --handlers

# Remove module
appa module remove nginx

# Edit module definition
appa module edit nginx

# Validate module structure
appa module validate nginx
appa module validate --all

# Test module configuration
appa module test nginx --config=./test-config.yaml
appa module test nginx --dry-run --backend=docker

# Show module usage
appa module show nginx --profiles
appa module show nginx --systems

# Tag management
appa module set nginx --tag+=web
appa module set nginx --tag+=proxy
appa module show nginx --tags
```

## Module Concept

Modules define **backend-agnostic configuration options** that can be deployed to any infrastructure backend (Docker, Kubernetes, NixOS, etc.) through plugin handlers.

### Module Structure
```
modules/
├── nginx/
│   ├── module.yaml          # Module definition & options schema
│   ├── handlers/            # Backend-specific transformation logic
│   │   ├── docker.py        # Docker deployment handler
│   │   ├── nix.py          # NixOS deployment handler
│   │   ├── k8s.py          # Kubernetes deployment handler
│   │   └── ansible.py      # Ansible deployment handler
│   ├── templates/          # Template files for transformations
│   │   ├── nginx.conf.j2
│   │   └── docker-compose.yml.j2
│   └── tests/              # Module validation tests
└── postgres/
    ├── module.yaml
    └── handlers/
```

### 80/20 Philosophy

Modules provide **80% coverage** for common use cases through clean options, with **20% fallback** to native backend configuration:

```yaml
# 80% - Clean module interface
modules:
  nginx:
    enable: true
    virtualHosts:
      "web.homelab.local":
        ssl:
          enable: true
          certificatePath: "secrets://ssl/web_cert"
        locations:
          "/":
            proxyPass: "http://localhost:3000"

# 20% - Native config fallback for advanced features
extraConfig:
  nix: |
    services.nginx.appendHttpConfig = ''
      geoip2 /usr/share/GeoIP/GeoLite2-Country.mmdb {
        $geoip2_data_country_code country iso_code;
      }
    '';
  docker: !include "./nginx/advanced-docker.yml"
  k8s: !include "./k8s/nginx-ingress.yaml"
```

## Module Definition Examples

### Simple Module (module.yaml)
```yaml
name: nginx
version: "1.0.0"
description: "Nginx web server configuration"

options:
  enable:
    type: bool
    default: false
    description: "Whether to enable nginx"

  user:
    type: str
    default: "nginx"
    description: "User to run nginx as"

  workerProcesses:
    type: either int str
    default: "auto"
    description: "Number of worker processes"

  virtualHosts:
    type: attrsOf:
      options:
        serverName:
          type: str
          description: "Server name for virtual host"
        ssl:
          type: submodule:
            options:
              enable:
                type: bool
                default: false
              certificatePath:
                type: nullOr path
                default: null
        locations:
          type: attrsOf:
            options:
              proxyPass:
                type: nullOr str
                default: null
              root:
                type: nullOr path
                default: null

handlers:
  - docker
  - nix
  - k8s
  - ansible

tags:
  - web
  - proxy
  - server
```

### Complex Module with Submodules
```yaml
name: postgres
version: "1.0.0"
description: "PostgreSQL database server"

options:
  enable:
    type: bool
    default: false

  version:
    type: str
    default: "15"
    description: "PostgreSQL version"

  databases:
    type: listOf:
      type: submodule:
        options:
          name:
            type: str
            description: "Database name"
          owner:
            type: str
            description: "Database owner"
          encoding:
            type: str
            default: "UTF8"

  users:
    type: listOf:
      type: submodule:
        options:
          name:
            type: str
          password:
            type: str
            description: "Password (supports secrets:// URIs)"
          databases:
            type: listOf str
            default: []

  settings:
    type: attrsOf (either str int bool)
    default: {}
    description: "PostgreSQL configuration settings"
    example:
      max_connections: 200
      shared_buffers: "256MB"
      effective_cache_size: "1GB"

handlers:
  - docker
  - nix
  - k8s

tags:
  - database
  - storage
```

## Handler Development

### Creating Module Handlers

```bash
# Generate handler template
appa module add-handler nginx --backend=podman
appa module add-handler postgres --backend=systemd

# Test handler implementation
appa module test-handler nginx --backend=docker --config=./test.yaml

# Validate handler output
appa module validate-handler nginx --backend=k8s --dry-run
```

### Handler Implementation Example
```python
# modules/nginx/handlers/docker.py
from appa.handlers import DockerHandler
from appa.types import ModuleConfig

class NginxDockerHandler(DockerHandler):
    def generate_config(self, module_config: ModuleConfig) -> dict:
        """Transform module config to docker-compose format"""

        if not module_config.options.get('enable', False):
            return {}

        compose = {
            'version': '3.8',
            'services': {
                'nginx': {
                    'image': 'nginx:alpine',
                    'user': module_config.options.get('user', 'nginx'),
                    'ports': self._generate_ports(module_config),
                    'volumes': self._generate_volumes(module_config),
                    'environment': self._generate_env(module_config)
                }
            }
        }

        return compose

    def _generate_ports(self, config):
        ports = ['80:80']

        # Add HTTPS port if any vhost has SSL
        for vhost in config.options.get('virtualHosts', {}).values():
            if vhost.get('ssl', {}).get('enable', False):
                ports.append('443:443')
                break

        return ports
```

## Module Usage in Profiles/Systems

### Profile Usage
```yaml
# profiles/web-server.yaml
modules:
  nginx:
    enable: true
    user: "www-data"
    virtualHosts:
      "web.homelab.local":
        serverName: "web.homelab.local"
        ssl:
          enable: true
          certificatePath: "secrets://ssl/web_cert"
        locations:
          "/":
            proxyPass: "http://localhost:3000"
          "/api":
            proxyPass: "http://localhost:8080"
```

### System Usage
```yaml
# systems/web-01.yaml
modules:
  nginx:
    enable: true
    virtualHosts:
      "web-01.homelab.local":
        ssl:
          enable: true
        locations:
          "/":
            root: "/var/www/html"
```

## Module Testing

### Validation Commands
```bash
# Test module options schema
appa module validate nginx --check=schema

# Test with sample configuration
appa module test nginx --config=./test-nginx.yaml

# Test handler transformations
appa module test nginx --backend=docker --show-output
appa module test nginx --backend=nix --dry-run

# Test across all supported backends
appa module test nginx --all-backends
```

### Test Configuration Example
```yaml
# test-nginx.yaml
modules:
  nginx:
    enable: true
    virtualHosts:
      "test.local":
        ssl:
          enable: false
        locations:
          "/":
            proxyPass: "http://localhost:3000"
```

## Common Flags

- `--description=DESC` - Module description
- `--tag+=TAG` - Add classification tag
- `--tag-=TAG` - Remove tag
- `--options` - Show module options schema
- `--handlers` - Show available backend handlers
- `--profiles` - Show profiles using this module
- `--systems` - Show systems using this module
- `--config=FILE` - Test configuration file
- `--backend=BACKEND` - Specific backend to test
- `--all-backends` - Test all supported backends
- `--dry-run` - Show generated config without deploying
- `--show-output` - Display generated backend configuration

## Module Lifecycle

### Development Workflow
```bash
# 1. Create module structure
appa module add myservice --description="Custom service module"

# 2. Define options schema
appa module edit myservice  # Edit module.yaml

# 3. Implement handlers
appa module add-handler myservice --backend=docker
appa module add-handler myservice --backend=nix

# 4. Test module
appa module test myservice --config=./test.yaml --all-backends

# 5. Validate and deploy
appa module validate myservice
appa system deploy test-system --module=myservice
```

## Best Practices

### Module Design
- **Start Simple**: Begin with 5-10 most common options
- **80/20 Rule**: Handle common cases via options, complex cases via extraConfig
- **Type Safety**: Use strong typing in option definitions
- **Documentation**: Provide clear descriptions and examples
- **Backwards Compatibility**: Version modules when making breaking changes

### Handler Implementation
- **Idempotent**: Handlers should produce same output for same input
- **Validated Output**: Ensure generated config is valid for target backend
- **Error Handling**: Provide clear error messages for invalid configurations
- **Testing**: Include comprehensive tests for all supported options

### Organization
- **Consistent Naming**: Use clear, descriptive module names
- **Logical Grouping**: Use tags to group related modules
- **Shared Components**: Extract common patterns into reusable submodules
- **Version Management**: Use semantic versioning for module releases

## See Also

- [Module Architecture](../02-architecture/plugin-system.md) - Technical implementation details
- [Profile Commands](cli-profile.md) - Using modules in profiles
- [System Commands](cli-system.md) - Using modules in systems
- [Plugin Development](../05-contributing/) - Creating custom handlers