# Module Interface and Plugin Handler System

This document defines the standard interface between Appa modules and plugin handlers, enabling clean separation between module definitions and their execution backends.

## Architecture Overview

```
modules/
├── web-server/
│   ├── appa.yaml              # Module definition
│   ├── configuration.nix       # Nix implementation
│   ├── playbook.yml            # Ansible implementation
│   └── compose.yml             # Docker implementation
├── database/
│   ├── appa.yaml
│   ├── flake.nix
│   └── deployment.yaml         # Kubernetes implementation
└── monitoring/
    ├── appa.yaml
    └── docker-compose.yml
```

## Module Definition Format

### Module Types

Modules can be defined in two formats:

1. **YAML Module** (`appa.yaml`) - Default format with explicit configuration
2. **Nix Flake Module** (`flake.nix`) - When nix flake support is enabled

### YAML Module Structure (appa.yaml)

```yaml
# Standard Appa object properties
name: web-server
description: "Nginx web server configuration with SSL support"
format_version: "1.0"

# Module-specific properties
type: module
handler: nix                    # Primary plugin handler
alternative_handlers:           # Optional: other supported handlers
  - ansible
  - docker

# Input declarations (flake-inspired)
inputs:
  nixpkgs:
    fetch: git
    url: "https://github.com/NixOS/nixpkgs"
    ref: "nixos-23.11"
    follows: []

  home-manager:
    fetch: git
    url: "https://github.com/nix-community/home-manager"
    ref: "release-23.11"
    follows: ["nixpkgs"]

  ssl-config:
    fetch: http
    url: "https://config.company.com/ssl/default.conf"
    checksum: "sha256:abcd1234..."

  monitoring-templates:
    fetch: appa-module
    module: "monitoring-base"
    version: ">=1.0"

  custom-packages:
    fetch: file
    path: "../packages"

# Output declarations
outputs:
  configurations:
    nixos-module: "configuration.nix"
    ansible-playbook: "playbook.yml"
    docker-compose: "compose.yml"

  packages:
    nginx-config: "templates/nginx.conf"
    ssl-certs: "certs/"

  schemas:
    variables: "schema/variables.json"
    system-requirements: "schema/requirements.json"

# Handler-specific configurations
handler_config:
  nix:
    file: "configuration.nix"
    type: "nixos-module"
    requires_flake: false
  ansible:
    file: "playbook.yml"
    type: "playbook"
    requirements: "requirements.yml"
  docker:
    file: "compose.yml"
    type: "docker-compose"

# Variable definitions for templating
variables:
  hostname:
    type: string
    required: true
    description: "System hostname"
  ip_address:
    type: string
    required: true
    pattern: "^\\d+\\.\\d+\\.\\d+\\.\\d+$"
    description: "IPv4 address"
  domain:
    type: string
    required: false
    default: "homelab.local"
    description: "Domain name"
  ssl_cert_path:
    type: string
    required: false
    secret: true
    description: "Path to SSL certificate"

# Dependencies on other modules
dependencies:
  - name: "ssl-certificates"
    version: ">=1.0"
  - name: "base-security"
    version: "*"

# Tags for module discovery
tags:
  - "tag:web"
  - "tag:nginx"
  - "tag:ssl"

# Maintainers
maintainers:
  - name: "Platform Team"
    email: "platform@company.com"
```

### Nix Flake Module Structure (flake.nix)

```nix
{
  description = "Web server configuration with SSL support";

  # Input declarations (standard Nix flake inputs)
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    home-manager = {
      url = "github:nix-community/home-manager/release-23.11";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Appa-specific inputs using fetch plugins
    ssl-config = {
      url = "http+https://config.company.com/ssl/default.conf";
      type = "file";
    };

    monitoring-templates = {
      url = "appa-module+monitoring-base";
      type = "appa-module";
    };
  };

  # Output declarations
  outputs = { self, nixpkgs, home-manager, ssl-config, monitoring-templates }:
  let
    # Appa metadata (embedded in flake)
    appa = {
      name = "web-server";
      description = "Nginx web server configuration with SSL support";
      type = "module";
      handler = "nix";
      alternative_handlers = [ "ansible" "docker" ];

      # Variable schema
      variables = {
        hostname = {
          type = "string";
          required = true;
          description = "System hostname";
        };
        domain = {
          type = "string";
          required = false;
          default = "homelab.local";
          description = "Domain name";
        };
      };

      tags = [ "tag:web" "tag:nginx" "tag:ssl" ];

      maintainers = [{
        name = "Platform Team";
        email = "platform@company.com";
      }];
    };
  in {
    # Standard flake outputs
    nixosModules.default = { config, lib, pkgs, ... }: {
      # NixOS module implementation
      services.nginx = {
        enable = true;
        virtualHosts."${config.appa.variables.hostname}" = {
          # Configuration using inputs
        };
      };
    };

    # Appa-specific outputs
    appaModule = {
      inherit appa;

      # Handler configurations
      handlers = {
        ansible = {
          file = "playbook.yml";
          template_vars = self.appa.variables;
        };
        docker = {
          file = "compose.yml";
          template_vars = self.appa.variables;
        };
      };

      # Output artifacts
      packages = {
        nginx-config = ./templates/nginx.conf;
        ssl-certs = ./certs;
      };
    };

    # Development shell
    devShells.default = nixpkgs.legacyPackages.x86_64-linux.mkShell {
      buildInputs = with nixpkgs.legacyPackages.x86_64-linux; [
        ansible
        docker
      ];
    };
  };
}
```

## Input System and Fetch Plugins

### Fetch Plugin Architecture

Fetch plugins provide standardized ways to retrieve external dependencies. Each fetch plugin handles a specific protocol or source type.

#### Available Fetch Plugins

##### Git Fetch Plugin
```yaml
inputs:
  nixpkgs:
    fetch: git
    url: "https://github.com/NixOS/nixpkgs"
    ref: "nixos-23.11"                # branch, tag, or commit
    follows: []                       # dependency following
    shallow: true                     # optional: shallow clone
    submodules: false                 # optional: include submodules

  # SSH git repository
  private-config:
    fetch: git
    url: "git@github.com:company/private-config"
    ref: "main"
    ssh_key: "secrets://git/deploy_key"
```

##### HTTP Fetch Plugin
```yaml
inputs:
  ssl-config:
    fetch: http
    url: "https://config.company.com/ssl/default.conf"
    method: "GET"                     # HTTP method
    headers:                          # optional headers
      Authorization: "Bearer secrets://api/token"
      User-Agent: "appa/1.0"
    checksum: "sha256:abcd1234..."    # verification
    timeout: 30                       # seconds

  # REST API data
  service-catalog:
    fetch: http
    url: "https://api.company.com/services"
    headers:
      Accept: "application/json"
    cache_ttl: 3600                   # cache for 1 hour
```

##### File Fetch Plugin
```yaml
inputs:
  # Local file/directory
  custom-packages:
    fetch: file
    path: "../packages"               # relative to module
    recursive: true                   # include subdirectories

  # Absolute path
  system-certs:
    fetch: file
    path: "/etc/ssl/certs"
    pattern: "*.pem"                  # optional: file filter
```

##### Appa Module Fetch Plugin
```yaml
inputs:
  # Reference another appa module
  monitoring-templates:
    fetch: appa-module
    module: "monitoring-base"
    version: ">=1.0"                  # version constraint
    outputs: ["templates", "configs"] # specific outputs

  # Local module reference
  base-config:
    fetch: appa-module
    module: "./base"                  # relative path
    outputs: ["*"]                    # all outputs
```

##### Archive Fetch Plugin
```yaml
inputs:
  # Download and extract archives
  external-tools:
    fetch: archive
    url: "https://releases.example.com/tools-v1.2.tar.gz"
    type: "tar.gz"                    # tar.gz, zip, tar.xz
    checksum: "sha256:..."
    extract_path: "tools/"            # extract to subdirectory
    strip_components: 1               # remove top-level directory
```

##### Container Registry Fetch Plugin
```yaml
inputs:
  # Container images
  base-image:
    fetch: container
    registry: "docker.io"
    image: "nginx"
    tag: "1.25-alpine"
    digest: "sha256:..."              # optional: specific digest
    platform: "linux/amd64"          # optional: platform
```

### Input Resolution Process

1. **Dependency Analysis**: Build dependency graph from inputs
2. **Fetch Execution**: Download/fetch inputs in dependency order
3. **Verification**: Validate checksums and signatures
4. **Caching**: Store fetched inputs in local cache
5. **Availability**: Make inputs available to module handlers

### Input Cache System

```yaml
# Cache configuration in .appa/config.toml
[cache]
directory = ".appa/cache"
max_size = "10GB"
ttl_default = 86400                   # 24 hours

[cache.fetch_plugins]
git_shallow = true
http_compression = true
cleanup_interval = 3600               # 1 hour
```

### Input Follows System

Similar to Nix flakes, inputs can follow other inputs to avoid version conflicts:

```yaml
inputs:
  nixpkgs:
    fetch: git
    url: "https://github.com/NixOS/nixpkgs"
    ref: "nixos-23.11"

  home-manager:
    fetch: git
    url: "https://github.com/nix-community/home-manager"
    ref: "release-23.11"
    follows: ["nixpkgs"]              # use same nixpkgs as parent

  custom-module:
    fetch: appa-module
    module: "custom-base"
    follows: ["nixpkgs", "home-manager"] # inherit multiple inputs
```

### Output System

Modules declare outputs that can be consumed by other modules or used by handlers:

```yaml
outputs:
  # Configuration files for different handlers
  configurations:
    nixos-module: "configuration.nix"
    ansible-playbook: "playbook.yml"
    docker-compose: "compose.yml"
    kubernetes-manifest: "deployment.yaml"

  # Reusable packages/artifacts
  packages:
    nginx-config: "templates/nginx.conf"
    ssl-certs: "certs/"
    scripts: "scripts/"

  # Schema definitions
  schemas:
    variables: "schema/variables.json"
    system-requirements: "schema/requirements.json"
    api: "schema/api.yaml"

  # Documentation
  docs:
    readme: "README.md"
    examples: "examples/"
    changelog: "CHANGELOG.md"
```

### Module Type Detection

Appa automatically detects module type based on files present:

```bash
# Detection logic
if flake.nix exists and nix_flake_enabled:
    module_type = "nix-flake"
elif appa.yaml exists:
    module_type = "yaml"
else:
    error "No valid module definition found"
```

### Cross-Reference Between Formats

YAML modules can reference Nix flake modules and vice versa:

```yaml
# In YAML module
inputs:
  nix-base:
    fetch: appa-module
    module: "nix-base"                # References a flake module
    type: "nix-flake"
```

```nix
# In Nix flake module
inputs = {
  yaml-config = {
    url = "appa-module+yaml-config";
    type = "appa-module";
  };
};
```

## Plugin Handler Interface

### Handler Registration

Plugin handlers register themselves with Appa and declare which module types they can process:

```yaml
# In .appa/plugins.toml
[plugins.nix]
enabled = true
handler_types = ["nixos-module", "nix-flake"]
executable = "appa-nix-handler"

[plugins.ansible]
enabled = true
handler_types = ["playbook", "role"]
executable = "appa-ansible-handler"

[plugins.docker]
enabled = true
handler_types = ["docker-compose", "dockerfile"]
executable = "appa-docker-handler"

[plugins.kubernetes]
enabled = true
handler_types = ["kubernetes-manifest", "helm-chart"]
executable = "appa-k8s-handler"
```

### Handler Execution Protocol

When applying a module to a system, Appa calls the handler with standardized input:

#### Input Format (JSON via stdin)

```json
{
  "context": {
    "operation": "apply",
    "dry_run": false,
    "verbose": true
  },
  "system": {
    "name": "web-01",
    "type": "server",
    "env": "prod",
    "hardware": {
      "cpu": 4,
      "memory": "8GB"
    },
    "network": {
      "ip": "10.0.1.10",
      "domain": "homelab.local",
      "fqdn": "web-01.homelab.local"
    },
    "tags": ["tag:server", "tag:prod", "tag:web"],
    "backend": "nix"
  },
  "module": {
    "name": "web-server",
    "path": "/path/to/modules/web-server",
    "handler": "nix",
    "handler_config": {
      "file": "configuration.nix",
      "type": "nixos-module"
    },
    "variables": {
      "hostname": "web-01",
      "ip_address": "10.0.1.10",
      "domain": "homelab.local",
      "ssl_cert_path": "/etc/ssl/certs/web-01.pem"
    }
  },
  "profile": {
    "name": "nginx-proxy",
    "variables": {
      "upstream_servers": ["10.0.1.20", "10.0.1.21"]
    }
  },
  "secrets": {
    "ssl_cert_path": "/etc/ssl/certs/web-01.pem",
    "ssl_key_path": "/etc/ssl/private/web-01.key"
  }
}
```

#### Output Format (JSON via stdout)

```json
{
  "status": "success",
  "message": "Module applied successfully",
  "changes": [
    {
      "type": "file",
      "path": "/etc/nginx/nginx.conf",
      "action": "modified"
    },
    {
      "type": "service",
      "name": "nginx",
      "action": "restarted"
    }
  ],
  "artifacts": {
    "config_files": [
      "/etc/nginx/nginx.conf",
      "/etc/nginx/sites-enabled/default"
    ],
    "log_file": "/var/log/appa/web-server-apply.log"
  },
  "metadata": {
    "execution_time": "2.5s",
    "handler_version": "1.2.0"
  }
}
```

### Handler Operations

Handlers must support these standard operations:

#### Apply Operation
```bash
appa-nix-handler apply < input.json
```

#### Validate Operation
```bash
appa-nix-handler validate < input.json
```

#### Plan Operation (dry-run)
```bash
appa-nix-handler plan < input.json
```

#### Remove Operation
```bash
appa-nix-handler remove < input.json
```

## Variable Substitution System

### Variable Sources

Variables are collected from multiple sources in priority order:

1. **System Properties**: Hardware, network, basic system info
2. **Profile Variables**: Variables defined in the profile applying the module
3. **Module Defaults**: Default values from module definition
4. **Environment Variables**: Runtime environment variables
5. **Secrets**: Values from secrets store

### Variable Processing

```yaml
# Module variable definition
variables:
  nginx_port:
    type: integer
    required: false
    default: 80
    min: 1
    max: 65535

  upstream_servers:
    type: array
    items:
      type: string
      pattern: "^\\d+\\.\\d+\\.\\d+\\.\\d+$"
    required: true

  ssl_enabled:
    type: boolean
    default: true

  cert_file:
    type: string
    secret: true
    required_when: "ssl_enabled == true"
```

### Template Processing

Handlers receive fully resolved variables but can also access templating functions:

```json
{
  "module": {
    "variables": {
      "hostname": "web-01",
      "fqdn": "web-01.homelab.local",
      "nginx_config": "server {\n  listen 80;\n  server_name web-01.homelab.local;\n}"
    },
    "template_functions": {
      "join": "function for joining arrays",
      "upper": "function for uppercase conversion",
      "base64": "function for base64 encoding"
    }
  }
}
```

## Handler Implementation Examples

### Nix Handler (appa-nix-handler)

```rust
// Pseudocode for Nix handler
fn main() {
    let input: HandlerInput = read_json_from_stdin();

    match input.context.operation {
        "apply" => apply_nix_module(input),
        "validate" => validate_nix_module(input),
        "plan" => plan_nix_module(input),
        "remove" => remove_nix_module(input),
    }
}

fn apply_nix_module(input: HandlerInput) -> HandlerOutput {
    // 1. Substitute variables in configuration.nix
    let config_content = substitute_variables(
        &input.module.path.join(&input.module.handler_config.file),
        &input.module.variables
    );

    // 2. Write temporary configuration
    let temp_config = write_temp_file(config_content);

    // 3. Build and apply configuration
    let result = Command::new("nixos-rebuild")
        .arg("switch")
        .arg("--flake")
        .arg(&temp_config)
        .arg("--target-host")
        .arg(&input.system.network.ip)
        .output();

    // 4. Return results
    HandlerOutput {
        status: if result.success() { "success" } else { "failed" },
        changes: detect_changes(&result),
        artifacts: collect_artifacts(),
    }
}
```

### Ansible Handler (appa-ansible-handler)

```python
# Pseudocode for Ansible handler
import json
import sys
from ansible.executor.playbook_executor import PlaybookExecutor

def main():
    input_data = json.load(sys.stdin)

    if input_data['context']['operation'] == 'apply':
        result = apply_ansible_playbook(input_data)
    elif input_data['context']['operation'] == 'validate':
        result = validate_ansible_playbook(input_data)

    json.dump(result, sys.stdout)

def apply_ansible_playbook(input_data):
    # 1. Create inventory with target system
    inventory = create_inventory(input_data['system'])

    # 2. Substitute variables in playbook
    playbook_path = substitute_variables(
        input_data['module']['path'] + '/' + input_data['module']['handler_config']['file'],
        input_data['module']['variables']
    )

    # 3. Execute playbook
    executor = PlaybookExecutor(
        playbooks=[playbook_path],
        inventory=inventory,
        variable_manager=create_variable_manager(input_data['module']['variables'])
    )

    result = executor.run()

    return {
        'status': 'success' if result == 0 else 'failed',
        'changes': collect_ansible_changes(),
        'artifacts': {'playbook_log': '/tmp/ansible.log'}
    }
```

## Error Handling

### Standard Error Codes

```json
{
  "status": "failed",
  "error": {
    "code": "VALIDATION_FAILED",
    "message": "Variable 'hostname' is required but not provided",
    "details": {
      "missing_variables": ["hostname"],
      "invalid_variables": []
    }
  }
}
```

### Error Code Categories

- **VALIDATION_FAILED**: Input validation errors
- **DEPENDENCY_MISSING**: Required dependencies not available
- **EXECUTION_FAILED**: Handler execution failed
- **PERMISSION_DENIED**: Insufficient permissions
- **NETWORK_ERROR**: Network connectivity issues
- **TIMEOUT**: Operation exceeded timeout

## Module Discovery and Validation

### Module Registry

Appa maintains a registry of available modules:

```bash
# List available modules
appa modules list

# Show module details
appa modules show web-server

# Validate module definition
appa modules validate web-server

# Test module against system
appa web-01 test-module web-server --dry-run
```

### Module Validation

Modules are validated for:

1. **Schema Compliance**: appa.yaml follows specification
2. **Handler Availability**: Required handlers are installed
3. **Variable Types**: Variable definitions are valid
4. **Dependencies**: Required modules are available
5. **File Existence**: Referenced files exist in module directory

## Benefits of This Design

1. **Clean Separation**: Module logic separate from execution backends
2. **Plugin Flexibility**: Easy to add new handler types
3. **Standardized Interface**: Consistent input/output across all handlers
4. **Variable Safety**: Type-checked variables with validation
5. **Handler Agnostic**: Modules can support multiple execution backends
6. **Testability**: Modules can be tested independently
7. **Reusability**: Modules work across different infrastructure setups