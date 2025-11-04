# Docker Plugin

The Docker plugin manages Docker containers and Docker Compose applications across your homelab infrastructure.

## Overview

- **Purpose**: Container orchestration and management
- **Features**: Docker Compose, container lifecycle, registry integration
- **Integration**: Works with other backends for host setup
- **Status**: Optional plugin for containerized applications

## Configuration

### Plugin Settings (.appa/plugins.toml)
```toml
[plugins.docker]
enabled = true
default_compose_version = "3.8"
registry_auth = true
cleanup_dangling = true
```

### System Configuration
```yaml
# In systems/toph.yaml (Docker host)
additional_tags: [tag:docker, tag:containers]

# Services running as containers
services:
  - name: gitlab
    port: 3000
    internal: true
    container: "gitlab/gitlab-ce"
    domain: "gitlab.avatar"
  - name: wikijs
    port: 3001
    internal: true
    container: "requarks/wiki"
    domain: "wiki.avatar"
```

## Features

### Container Management
- **Lifecycle**: Start, stop, restart, remove containers
- **Health Checks**: Monitor container health status
- **Log Management**: Centralized container logging
- **Resource Limits**: CPU, memory, and storage constraints

### Docker Compose Integration
- **Multi-Service Apps**: Define complex applications
- **Service Dependencies**: Manage startup order and dependencies
- **Environment Variables**: Inject configuration via environment
- **Volume Management**: Persistent storage and bind mounts

### Registry Integration
- **Private Registries**: Support for private Docker registries
- **Authentication**: Registry login and credential management
- **Image Management**: Pull, build, and push container images
- **Security Scanning**: Integrate with image security scanners

## Module Structure

### Docker Compose Module
```yaml
# modules/docker/gitlab/docker-compose.yml
version: '3.8'

services:
  gitlab:
    image: gitlab/gitlab-ce:latest
    hostname: gitlab.avatar
    container_name: gitlab
    restart: unless-stopped

    environment:
      GITLAB_OMNIBUS_CONFIG: |
        external_url 'https://gitlab.avatar'
        nginx['listen_port'] = 3000
        nginx['listen_https'] = false
        gitlab_rails['initial_root_password'] = '${GITLAB_ROOT_PASSWORD}'

    ports:
      - "3000:3000"
      - "2222:22"

    volumes:
      - gitlab_config:/etc/gitlab
      - gitlab_logs:/var/log/gitlab
      - gitlab_data:/var/opt/gitlab

    networks:
      - homelab

  volumes:
    gitlab_config:
    gitlab_logs:
    gitlab_data:

  networks:
    homelab:
      external: true
```

### Environment Configuration
```bash
# modules/docker/gitlab/.env
GITLAB_ROOT_PASSWORD=secrets://gitlab/root_password
POSTGRES_PASSWORD=secrets://gitlab/postgres_password
REDIS_PASSWORD=secrets://gitlab/redis_password
```

### Module Definition
```yaml
# Module metadata
name: gitlab-docker
description: "GitLab CE container deployment"
backend: docker
file_path: "modules/docker/gitlab/docker-compose.yml"

dependencies:
  - docker-host-setup

variables:
  - name: domain
    type: string
    default: "gitlab.localhost"
    description: "Domain for GitLab instance"
  - name: root_password
    type: secret
    description: "Initial root password"
```

## CLI Commands

### Container Operations
```bash
# Container lifecycle
appa toph docker ps                # List running containers
appa toph docker start gitlab     # Start specific container
appa toph docker stop gitlab      # Stop specific container
appa toph docker restart gitlab   # Restart specific container
appa toph docker logs gitlab      # Show container logs

# Container management
appa toph docker exec gitlab bash # Execute command in container
appa toph docker inspect gitlab   # Show container details
```

### Compose Operations
```bash
# Compose management
appa toph docker compose up       # Start all compose services
appa toph docker compose down     # Stop all compose services
appa toph docker compose ps       # Show compose service status
appa toph docker compose logs     # Show compose logs

# Service-specific operations
appa toph docker compose start gitlab
appa toph docker compose stop gitlab
appa toph docker compose restart gitlab
```

### Image Management
```bash
# Image operations
appa toph docker images            # List local images
appa toph docker pull gitlab/gitlab-ce
appa toph docker build --tag=myapp .
appa toph docker push myregistry/myapp

# Cleanup operations
appa toph docker cleanup          # Remove dangling images/containers
appa toph docker prune            # Prune unused resources
```

## Integration Points

### With Ansible Plugin
1. **Host Setup**: Ansible installs Docker and configures host
2. **Docker Plugin**: Manages container deployment and lifecycle
3. **Updates**: Docker plugin updates containers, Ansible updates host

### With Proxmox Plugin
1. **VM Creation**: Proxmox creates VM for container host
2. **Base Setup**: Ansible/Nix configures Docker host
3. **Container Deployment**: Docker plugin deploys applications

### With Secrets Plugin
```yaml
# Environment file with secret references
GITLAB_ROOT_PASSWORD=secrets://gitlab/root_password
DATABASE_URL=secrets://gitlab/database_url
```

## Service Discovery

### Nginx Integration
Docker services can be automatically configured with nginx proxy:

```yaml
# In nginx configuration
services:
  - name: gitlab
    container: gitlab
    domain: gitlab.avatar
    port: 3000
    ssl: true
```

### Network Configuration
```yaml
# Docker network setup
networks:
  homelab:
    driver: bridge
    ipam:
      config:
        - subnet: 172.20.0.0/16
```

## Configuration Patterns

### Multi-Service Application
```yaml
version: '3.8'
services:
  web:
    image: myapp:latest
    depends_on:
      - database
      - redis
    environment:
      DATABASE_URL: postgresql://user:pass@database:5432/myapp
      REDIS_URL: redis://redis:6379

  database:
    image: postgres:14
    environment:
      POSTGRES_DB: myapp
      POSTGRES_PASSWORD: secrets://myapp/db_password
    volumes:
      - db_data:/var/lib/postgresql/data

  redis:
    image: redis:7
    command: redis-server --requirepass secrets://myapp/redis_password
```

### Health Checks
```yaml
services:
  web:
    image: nginx
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
```

### Resource Limits
```yaml
services:
  web:
    image: myapp
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 1G
        reservations:
          cpus: '1'
          memory: 512M
```

## Best Practices

### Security
- **Non-Root Containers**: Use non-root users in containers
- **Secret Management**: Store secrets in secrets.yaml, not compose files
- **Network Isolation**: Use custom networks for service isolation
- **Image Scanning**: Regularly scan images for vulnerabilities

### Performance
- **Resource Limits**: Set appropriate CPU and memory limits
- **Volume Management**: Use named volumes for persistent data
- **Image Optimization**: Use multi-stage builds and minimal base images
- **Health Checks**: Implement proper health checks for reliability

### Maintenance
- **Regular Updates**: Keep container images updated
- **Log Rotation**: Configure log rotation for containers
- **Backup Strategy**: Backup persistent volumes and configurations
- **Monitoring**: Monitor container resource usage and health

## Examples

See the [sample homelab modules](../../sample/homelab/modules/docker/) for complete Docker configuration examples.