# Plugin Commands

Plugin operations manage runtime-loadable extensions that add new subcommands to the Appa CLI.

## Plugin Operations

```bash
# Add (load) new plugin
appa plugin add sops --source=github.com/appa/plugins/sops
appa plugin add proxmox --source=local --path=/path/to/plugin

# List plugins
appa plugin list
appa plugin list --loaded
appa plugin list --available

# Show plugin information
appa plugin show sops
appa plugin show proxmox --commands

# Remove (unload) plugin
appa plugin remove sops
appa plugin remove proxmox --force

# Enable/disable plugins
appa plugin set sops --enabled=true
appa plugin set proxmox --enabled=false

# Validate plugin
appa plugin validate sops
appa plugin validate --all
appa plugin test proxmox --dry-run

# Update plugins
appa plugin update sops
appa plugin update --all
```

## Plugin-Specific Commands

When plugins are loaded, they extend the CLI with new subcommands:

```bash
# SOPS plugin commands
appa sops encrypt secrets.yaml
appa sops decrypt secrets.yaml
appa sops edit secrets.yaml

# Proxmox plugin commands
appa proxmox vm create --template=ubuntu-22.04
appa proxmox vm start vm-01
appa proxmox vm stop vm-01

# BMC plugin commands
appa bmc power on server-01
appa bmc power off server-01
appa bmc console server-01
```

## Plugin Sources

- **github.com/appa/plugins/** - Official plugin repository
- **local** - Local plugin development
- **git** - Custom Git repositories
- **registry** - Plugin registry

## Plugin Properties

- **name** - Unique plugin identifier
- **version** - Plugin version
- **source** - Installation source
- **enabled** - Whether plugin is active
- **commands** - Subcommands provided by plugin

## Common Flags

- `--source=SOURCE` - Plugin source location
- `--path=PATH` - Local plugin path
- `--version=VERSION` - Specific version to install
- `--enabled=BOOL` - Enable/disable plugin
- `--force` - Force operation without confirmation
- `--loaded` - Show only loaded plugins
- `--available` - Show available plugins
- `--commands` - Show plugin commands

## See Also

- [Plugin Architecture](../03-plugins/) - Plugin system overview
- [Plugin Development](../05-contributing/) - Creating custom plugins