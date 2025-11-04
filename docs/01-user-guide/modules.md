# Working with Modules

Modules are the heart of Appa's backend-agnostic configuration system. They let you define service configurations once and deploy them anywhere - whether as Docker containers, Kubernetes pods, NixOS services, or any other backend.

## Key Concepts

### What are Modules?

Think of modules like **universal configuration interfaces** for services. Similar to how NixOS modules work, but extending across any deployment backend:

- **nginx module** - Configure web servers the same way whether deploying to Docker, K8s, or NixOS
- **postgres module** - Set up databases with the same options across any platform
- **monitoring module** - Deploy observability stacks consistently everywhere

### The 80/20 Philosophy

Modules follow the **80/20 rule**:

- **80%** of use cases handled through clean, typed configuration options
- **20%** of complex scenarios handled by falling back to native backend config

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

# 20% - Advanced features via native config
extraConfig:
  nix: |
    services.nginx.appendHttpConfig = ''
      # Advanced nginx features not in module
      geoip2 /usr/share/GeoIP/GeoLite2-Country.mmdb {
        $geoip2_data_country_code country iso_code;
      }
    '';
  docker: !include "./nginx/advanced-config.yml"
```

## Common Workflows

### Configuring a Web Server

Here's how you'd set up nginx across different environments:

```yaml
# In a profile or system configuration
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
          keyPath: "secrets://ssl/web_key"
        locations:
          "/":
            proxyPass: "http://localhost:3000"
          "/api":
            proxyPass: "http://localhost:8080"
          "/static":
            root: "/var/www/static"
```

**The magic**: This same configuration works whether you deploy to:
- **Docker** (generates docker-compose.yml)
- **Kubernetes** (generates Deployment + Service + Ingress)
- **NixOS** (generates services.nginx config)
- **Ansible** (generates nginx playbooks)

### Database Setup

Configure PostgreSQL the same way everywhere:

```yaml
modules:
  postgres:
    enable: true
    version: "15"
    databases:
      - name: "myapp"
        owner: "appuser"
        encoding: "UTF8"
    users:
      - name: "appuser"
        password: "secrets://db/appuser_password"
        databases: ["myapp"]
    settings:
      max_connections: 200
      shared_buffers: "256MB"
      effective_cache_size: "1GB"
```

### Monitoring Stack

Deploy Prometheus + Grafana consistently:

```yaml
modules:
  prometheus:
    enable: true
    retention: "30d"
    scrapeConfigs:
      - job_name: "node-exporter"
        static_configs:
          - targets: ["localhost:9100"]

  grafana:
    enable: true
    adminPassword: "secrets://monitoring/grafana_admin"
    datasources:
      - name: "Prometheus"
        type: "prometheus"
        url: "http://localhost:9090"
```

## Module Organization Strategies

### Profile-Based Organization

Use profiles to group related modules:

```yaml
# profiles/web-server.yaml
modules:
  nginx:
    enable: true
    # ... nginx config

  node-exporter:
    enable: true
    # Monitoring for web servers

  fail2ban:
    enable: true
    # Security for web servers
```

```yaml
# profiles/database-server.yaml
modules:
  postgres:
    enable: true
    # ... postgres config

  postgres-exporter:
    enable: true
    # Database monitoring

  backup:
    enable: true
    # Database backups
```

### System-Specific Overrides

Override module settings per system:

```yaml
# systems/web-01.yaml (production)
profiles:
  - web-server

modules:
  nginx:
    # Override for production
    workerProcesses: 4
    virtualHosts:
      "web-01.homelab.local":
        ssl:
          certificatePath: "secrets://ssl/prod_cert"
```

```yaml
# systems/web-dev.yaml (development)
profiles:
  - web-server

modules:
  nginx:
    # Override for development
    workerProcesses: 1
    virtualHosts:
      "dev.homelab.local":
        ssl:
          enable: false  # No SSL in dev
```

## Backend Flexibility

The power of modules is **backend independence**. You can switch deployment methods without changing your service configuration.

### Same Config, Different Backends

```bash
# Deploy as Docker containers
appa system add web-docker --backend=docker --profile=web-server

# Deploy as NixOS services
appa system add web-nix --backend=nix --profile=web-server

# Deploy as Kubernetes pods
appa system add web-k8s --backend=k8s --profile=web-server
```

All three systems use the **same module configuration** but deploy completely differently under the hood.

### Migration Example

Moving from Docker to Kubernetes:

```bash
# 1. Current Docker deployment
appa system show web-01  # Shows backend=docker

# 2. Change backend (config stays the same!)
appa system set web-01 --backend=k8s

# 3. Deploy with new backend
appa system deploy web-01 --validate
```

Your nginx module configuration doesn't change - only the generated output changes from docker-compose.yml to Kubernetes manifests.

## Working with Complex Configurations

### File References

Modules can reference files in your project:

```yaml
modules:
  nginx:
    enable: true
    virtualHosts:
      "app.homelab.local":
        # Reference project files directly
        ssl:
          certificatePath: "./certs/app.crt"
          keyPath: "./certs/app.key"
        locations:
          "/":
            # Include custom nginx snippets
            extraConfig: !include "./nginx/proxy-headers.conf"
```

### Advanced Configurations

For the 20% of complex use cases, fall back to native config:

```yaml
modules:
  postgres:
    enable: true
    # Standard 80% config via module options
    version: "15"
    databases:
      - name: "myapp"
        owner: "appuser"

# Complex 20% cases via native config
extraConfig:
  nix: |
    # Advanced NixOS PostgreSQL setup
    services.postgresql = {
      settings = {
        shared_preload_libraries = "pg_stat_statements,auto_explain";
        auto_explain.log_min_duration = "1s";
      };

      # Custom initialization
      initialScript = pkgs.writeText "init.sql" ''
        CREATE EXTENSION IF NOT EXISTS postgis;
        CREATE EXTENSION IF NOT EXISTS pg_trgm;
      '';
    };

  docker: |
    # Advanced Docker setup
    version: '3.8'
    services:
      postgres-advanced:
        image: postgis/postgis:15-3.3
        volumes:
          - "./custom-init:/docker-entrypoint-initdb.d"
        environment:
          - "POSTGRES_INITDB_ARGS=--auth-host=scram-sha-256"
```

## Testing and Development

### Testing Module Configurations

```bash
# Test module config without deploying
appa module test nginx --config=./test-nginx.yaml --dry-run

# Test across different backends
appa module test nginx --config=./test-nginx.yaml --backend=docker
appa module test nginx --config=./test-nginx.yaml --backend=k8s

# Test all supported backends
appa module test nginx --config=./test-nginx.yaml --all-backends
```

### Incremental Development

Start simple and add complexity:

```yaml
# Step 1: Basic nginx setup
modules:
  nginx:
    enable: true
    virtualHosts:
      "test.local":
        locations:
          "/":
            root: "/var/www/html"
```

```yaml
# Step 2: Add SSL
modules:
  nginx:
    enable: true
    virtualHosts:
      "test.local":
        ssl:
          enable: true
          certificatePath: "secrets://ssl/test_cert"
        locations:
          "/":
            root: "/var/www/html"
```

```yaml
# Step 3: Add reverse proxy
modules:
  nginx:
    enable: true
    virtualHosts:
      "test.local":
        ssl:
          enable: true
          certificatePath: "secrets://ssl/test_cert"
        locations:
          "/":
            proxyPass: "http://localhost:3000"
          "/static":
            root: "/var/www/static"
```

## Integration with Other Objects

### Profiles Reference Modules

Profiles bundle related modules together:

```yaml
# profiles/full-stack-web.yaml
description: "Complete web application stack"

modules:
  nginx:
    enable: true
    # ... web server config

  postgres:
    enable: true
    # ... database config

  redis:
    enable: true
    # ... cache config

  monitoring:
    enable: true
    # ... observability config
```

### Systems Override Module Settings

Systems can customize module behavior:

```yaml
# systems/web-prod-01.yaml
profiles:
  - full-stack-web

# Production-specific overrides
modules:
  nginx:
    workerProcesses: 8
    clientMaxBodySize: "100M"

  postgres:
    settings:
      max_connections: 500
      shared_buffers: "2GB"
```

### Secrets Integration

Modules seamlessly work with encrypted secrets:

```yaml
modules:
  postgres:
    users:
      - name: "appuser"
        password: "secrets://db/appuser_password"

  nginx:
    virtualHosts:
      "secure.homelab.local":
        ssl:
          certificatePath: "secrets://ssl/secure_cert"
          keyPath: "secrets://ssl/secure_key"
```

## Best Practices

### Module Selection

- **Use existing modules first** - Check what's available before creating custom ones
- **Combine modules thoughtfully** - Related services (nginx + app) work well together
- **Profile organization** - Group modules by role (web-server, database, monitoring)

### Configuration Strategy

- **Start with defaults** - Most modules have sensible defaults for common cases
- **Environment-specific overrides** - Use system configs to customize per environment
- **Gradual complexity** - Begin simple, add advanced features as needed

### Backend Strategy

- **Start with what you know** - Use familiar backends initially (Docker, etc.)
- **Plan for migration** - Modules make switching backends easier later
- **Test across backends** - Validate your configs work on multiple platforms

### Development Workflow

1. **Configure in staging** - Test module configs in non-production first
2. **Validate generated output** - Check what backends actually generate
3. **Gradual rollout** - Deploy to one system, then expand
4. **Monitor and iterate** - Watch for issues, refine configurations

## Troubleshooting

### Common Issues

```bash
# Module not found
appa module list  # Check available modules
appa module show nginx --options  # Check module options

# Configuration validation errors
appa module validate nginx --config=./test.yaml
appa module test nginx --config=./test.yaml --dry-run

# Backend-specific problems
appa module test nginx --backend=docker --show-output
appa system deploy web-01 --dry-run --verbose
```

### Debug Generated Configuration

```bash
# See what backend config gets generated
appa module test nginx --backend=docker --show-output
appa module test nginx --backend=nix --show-output

# Compare across backends
appa module test nginx --all-backends --show-output
```

## See Also

- **[Module Commands Reference](../04-reference/cli-module.md)** - Complete command options
- **[Profiles](profile.md)** - Grouping modules together
- **[Systems](managing-systems.md)** - Deploying modules to infrastructure
- **[Secrets](secret.md)** - Using encrypted values in modules
- **[Plugin Architecture](../02-architecture/plugin-system.md)** - How module handlers work