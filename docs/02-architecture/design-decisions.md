# Design Decisions

This document captures key design decisions made during the development of Appa, including rationale and alternatives considered.

## CLI Command Structure

**Decision:** Use consistent `<object> <verb> [name]` command structure
- **Object operations**: `appa <object> <verb> [name]` - All operations under object type subcommands
- **Positional arguments**: Object names are positional arguments, not subcommands

**Rationale:**
- Object operations provide context: `appa system add web-01`
- Object-specific commands have tailored flags and behavior
- Object names as positional arguments enable object-specific defaults
- Consistent pattern simplifies learning and tooling
- Configuration is treated as an object: `appa config set`

**Command Design Rules:**

**Subcommand Structure:**
- Object operations: `appa <object> <verb> [name]` (e.g., `appa system add web-01`, `appa system set web-01 --env=staging`)
- Object names are positional arguments, not subcommands
- Configuration is treated as an object: `appa config set`

**Naming Guidelines:**
- Use simple, consistent verbs: `show`, `add`, `remove`, `set`, `edit`, `validate`, `deploy`, `apply`
- No hyphenated subcommands: use flags instead (`set --tag+=critical` not `add-tag`)
- Use positional arguments for names when unambiguous
- Prefer `set` over `update` for property modification
- Use `remove` not `delete` or `rm` for clarity
- Use `+=` and `-=` operators for adding/removing list items: `--tag+=critical`, `--profile-=old`

**Flag Style:**
- Use `--key=value` format for clarity
- When only one argument possible, use positional: `show profiles` not `show --profiles`
- Avoid abbreviated flags in examples (prefer `--env` over `-e`)

## Architecture Decisions

**Single Files Per System:**
- **Clarity**: One YAML file per system
- **VCS workflow**: Clean diffs and minimal merge conflicts
- **Portability**: Supports multiple formats (YAML/JSON/TOML)

**Single policy.yaml:**
- Access rules and profile policies in one place
- Centralized policy management and auditing
- Easier to validate policy consistency

**Global inventory.yaml:**
- Fast entity discovery without filesystem scanning
- Cross-reference validation and relationship mapping
- Global metadata and statistics collection

**Explicit Profile Assignment:**
- Prevents accidental configuration changes
- Clear audit trail of what profiles are applied
- Safer than automatic tag-based application

## Configuration Philosophy

**Opinionated Defaults:**
- YAML as default format (human-readable, VCS-friendly)
- Mesh networking assumptions (direct connectivity)
- Auto-tagging based on system type/environment
- Tailscale-inspired ACL system
- Standard directory structure

**Extensible Configuration:**
- Multiple format support (YAML/JSON/TOML)
- Pluggable backends (Nix, Ansible, Docker, K8s)
- Customizable templates
- Configurable tag schemas
- Flexible module organization

**Pure Declarative Configs:**
- No conditional logic in configuration files
- All logic in application code
- Easy to port between formats
- Simple to validate and parse

## User Management Decision

**Decision:** Appa manages infrastructure, not people
- People are managed in LDAP/FreeIPA
- One-way sync from appa to LDAP for groups and service accounts
- Appa is source of truth for infrastructure, LDAP for users

**Rationale:**
- Avoid duplicating user management functionality
- Leverage existing LDAP infrastructure
- Clear separation of concerns
- Simplifies the tool's scope

## Profile Application Safety

**Decision:** Explicit profile assignment over automatic tag-based application

**Rationale:**
- Tag-based automatic application could be dangerous
- Prevents accidental profile application
- Mandatory policies can still enforce required profiles
- Clear audit trail of what was applied when

## Policy Consolidation

**Decision:** All policies in single policy.yaml file

**Rationale:**
- Simpler than separate policy files per profile
- Centralized policy management
- Easier to validate consistency across policies
- Profile enforcement policies alongside access control rules

## Default Configuration Format

**Decision:** YAML as recommended default

**Rationale:**
- **Comments support**: Essential for documenting infrastructure decisions
- **Human readability**: Readable and editable in text editors
- **Multi-line strings**: Natural fit for SSH keys, scripts, and certificates
- **Industry standard**: Familiar to DevOps professionals (Kubernetes, Ansible, Docker Compose)
- **Complex data structures**: Supports nested configurations
- **VCS-friendly**: Produces readable diffs for change tracking

**Alternatives considered:**
- JSON: Good for programmatic access, poor for human editing
- TOML: Good for configuration, limited nested structure support

## Plugin Architecture Decisions

**Decision:** Use extensible plugin system for specialized functionality

**Rationale:**
- Keep core tool focused and lightweight
- Allow community contributions for specific backends
- Avoid vendor lock-in to specific tools
- Enable gradual adoption (use what you need)

### BMC Plugin Decision

**Decision:** Separate BMC plugin for bare metal provisioning

**Rationale:**
- Bare metal provisioning is complex and hardware-specific
- Not all homelabs need bare metal automation
- Allows integration with various BMC vendors (IPMI, Redfish)
- Provides foundation for other backends (installs base OS, then hands off)

### SOPS Plugin Decision

**Decision:** Use SOPS plugin as default secrets backend with pluggable alternatives

**Rationale:**
- SOPS is mature and widely adopted
- Supports multiple encryption backends (age, GPG)
- VCS-friendly (encrypted files can be committed)
- Allows for alternative secret backends (Vault, 1Password, etc.)

### Proxmox Plugin Structure

**Decision:** Differentiate `host:` vs `vm:` configuration sections

**Rationale:**
- Clear separation between hypervisor management and VM lifecycle
- Same plugin handles both host setup and VM operations
- Host config: cluster, storage, auth, API settings
- VM config: VM ID, template, node assignment, resources

### Tag-Based Plugin Integration

**Decision:** Use tags instead of backend field for plugin capabilities

**Rationale:**
- `proxmox-host` indicates system provides Proxmox capabilities
- `bare-metal` indicates BMC plugin should manage hardware
- Allows systems to have multiple plugin integrations
- Plugin can query for relevant systems by tags
- More flexible than single backend field

## Secrets Management Integration

**Decision:** Reference secrets using `secrets://` URI scheme

**Rationale:**
- Clear indication that value comes from secrets backend
- Allows different secret types: `secrets://ssh/admin`, `secrets://proxmox/api_token`
- Plugin-agnostic reference format
- Secure by default (no secret values in config files)

## Multi-Backend Workflow

**Decision:** BMC and PXE plugins bootstrap systems, then hand off to configuration management

**Workflow:**
1. BMC plugin manages hardware power and boot order
2. PXE plugin handles network boot and OS installation
3. PXE plugin ensures SSH access and installs base packages
4. Configuration management plugins (Ansible/Nix/Docker) take over for full configuration
5. Proxmox plugin manages hypervisor-specific setup

**Rationale:**
- Clear separation of concerns between provisioning and configuration
- BMC focuses on hardware power management
- PXE focuses on OS installation and bootstrap
- Configuration backends focus on service deployment
- Allows mixing of provisioning and configuration tools

## Plugin Management Decisions

**Decision:** Centralized plugin configuration in `.appa/plugins.toml`

**Rationale:**
- Single file for all plugin settings and enablement
- Version control for plugin configurations
- Clear separation from user preferences (config.toml)
- Allows per-plugin configuration sections

**Plugin Repository System:**
- Official repository for core/supported plugins
- Community repository for contributed plugins
- Version pinning and update management
- Plugin testing and validation framework

**Plugin Lifecycle:**
- Install: Download and register plugin
- Configure: Set plugin-specific settings
- Enable/Disable: Control plugin activation
- Update: Manage plugin versions
- Test: Validate plugin functionality

**Security Considerations:**
- Plugin signature verification (future)
- Sandboxed plugin execution (future)
- Permission-based plugin access
- Audit trail for plugin operations

## Global Naming Decision

**Decision:** Enforce global uniqueness for all appa object names

**Rationale:**
- Simplifies CLI commands - use name as positional argument instead of paths
- Prevents confusion between objects of different types
- Makes cross-references unambiguous
- Enables `appa <object> show <name>` command for any object type

**Naming Convention:**
- Systems: descriptive hostnames (`kyoshi`, `aang`, `toph`, `iroh`)
- Profiles: purpose-based names (`nginx-proxy`, `headscale`, `docker-host`)
- Modules: backend-purpose format (`web-server-nix`, `web-server-ansible`)
- Policies: descriptive policy names (`production-monitoring`, `ssh-access`)

**Show Command Structure:**
- `appa <object> show <name>` - shows basic object properties (name, type, description, tags, etc.)
- `appa <name> show` - shows detailed object-specific information based on type

**Examples:**
- `appa system show kyoshi` - shows basic properties: name, type, description, env, tags
- `appa kyoshi show` - shows detailed system info: hardware, network, services, profiles
- `appa profile show nginx-proxy` - shows basic properties: name, description, applies_to
- `appa nginx-proxy show` - shows detailed profile info: configurations, modules, variables
- `appa module show web-server-nix` - shows basic properties: name, description, backend
- `appa web-server-nix show` - shows detailed module info: file contents, dependencies

**Benefits:**
- Object-specific `appa <object> show <name>` for quick object identification
- Type-specific `appa <name> show` for detailed inspection
- No path arguments needed anywhere
- Clear separation between basic properties and detailed info