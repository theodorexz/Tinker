# Tinker

**A standalone Minecraft server management platform --- in
development.**

Tinker aims to make creating, configuring, running, and maintaining
Minecraft servers straightforward, whether a server lives on your own
computer or on another machine you manage.

The long-term goal is to build a complete server manager rather than
depend on Crafty Controller. Tinker is being redesigned around a desktop
interface, a Go-based management backend, and optional secure remote
connectivity.

> **Project status:** Early development and architecture planning. The
> roadmap describes intended functionality, not features that are
> already implemented. APIs, architecture, and supported platforms may
> change.

## Vision

Tinker aims to bring common Minecraft server-management tasks into one
application:

-   **Create servers:** Choose a Minecraft version, server type, and
    supported mod loader.
-   **Manage server lifecycle:** Start, stop, restart, monitor, and
    interact with server consoles.
-   **Manage modpacks:** Explore importing and managing Modrinth packs,
    including checking Minecraft versions, loaders, dependencies, and
    client/server mod compatibility.
-   **Manage files and backups:** Organize configurations, server files,
    backups, and multiple server instances.
-   **Explore worlds:** Build a world-map feature that loads map regions
    progressively as users pan and zoom.
-   **Manage remote machines:** Work toward secure remote access without
    requiring users to manually configure a VPN.
-   **Provide a desktop experience:** Use a component-based interface
    that can grow as Tinker gains features.

These are project goals. Support for specific Minecraft versions,
loaders, modpack workflows, operating systems, and remote-access
providers will be documented as they become available.

## Planned technology

  --------------------------------------------------------------------------------------
  Area                    Planned technology                     Responsibility
  ----------------------- -------------------------------------- -----------------------
  Desktop UI              [Avalonia                              Desktop interface
                          UI](https://avaloniaui.net/), C#, and  
                          XAML                                   

  Management backend      [Go](https://go.dev/)                  Server lifecycle,
                                                                 filesystem operations,
                                                                 downloads,
                                                                 integrations, and local
                                                                 API

  Modpack integration     [Modrinth                              Project/version
                          API](https://docs.modrinth.com/api/)   metadata and modpack
                                                                 workflows

  Data storage            To be decided; SQLite is a candidate   Local server metadata
                                                                 and settings

  Remote connectivity     To be designed                         Authenticated
                                                                 communication with
                                                                 remote Tinker agents

  World map               To be designed                         World processing, map
                                                                 tiles, caching, and
                                                                 incremental updates
  --------------------------------------------------------------------------------------

The UI and backend are intended to have clear responsibilities. Avalonia
will present information and user controls; Go will handle
server-management operations. The exact communication boundary between
them will be settled during prototyping.

## Architecture direction

The intended architecture has three main parts:

1.  **Desktop application** --- the interface used to configure and
    manage servers.
2.  **Tinker engine** --- the Go-based functionality that performs
    server-management tasks on the local machine.
3.  **Remote agent and connection service (planned)** --- a future way
    to manage servers on other machines through authenticated
    connections.

Local server management is the first priority. Remote connectivity
should be added after the local engine works reliably. Basic local
server management should not require a cloud service.

## Roadmap

The roadmap is a guide, not a promise of release dates.

### Foundation

-   [ ] Set up the Go project and module structure.
-   [ ] Create and load server configuration files.
-   [ ] Define core data models and error-handling conventions.
-   [ ] Add automated tests for core logic.
-   [ ] Prototype the Avalonia desktop interface.
-   [ ] Define and test the UI-to-engine communication boundary.

### Server management

-   [ ] Detect compatible Java installations.
-   [ ] Install and configure a vanilla Minecraft server.
-   [ ] Start and stop server processes safely.
-   [ ] Capture console output and send console commands.
-   [ ] Report server status and startup failures.
-   [ ] Support server configuration editing.
-   [ ] Add backup and restore workflows.

### Versions, loaders, and modpacks

-   [ ] Retrieve supported Minecraft version metadata.
-   [ ] Design a loader integration model.
-   [ ] Add loader support incrementally.
-   [ ] Explore Modrinth pack import and management.
-   [ ] Check version, loader, dependency, and client/server mod
    compatibility.
-   [ ] Provide clear diagnostics when a pack or server configuration is
    incompatible.

### World map

-   [ ] Research Minecraft world formats and rendering approaches.
-   [ ] Prototype rendering a small world region.
-   [ ] Explore tiled rendering, zoom levels, and caching.
-   [ ] Investigate updating only map regions that have changed.

### Remote management and release readiness

-   [ ] Design device registration and authentication.
-   [ ] Prototype secure remote communication.
-   [ ] Evaluate relay/tunnel options and operating costs.
-   [ ] Document permissions, secrets handling, and the threat model.
-   [ ] Add installation, upgrade, recovery, and troubleshooting
    documentation.
-   [ ] Test supported operating systems and Minecraft configurations.

## Development

Tinker is currently in its planning and prototyping stage. The final
repository layout and development commands will be documented after the
Avalonia and Go projects are scaffolded.

### Planned development prerequisites

-   [Go](https://go.dev/doc/install)
-   [.NET SDK](https://dotnet.microsoft.com/download)
-   [Avalonia documentation](https://docs.avaloniaui.net/)
-   [Visual Studio Code](https://code.visualstudio.com/) or another
    compatible editor
-   Git

Once the initial project structure is in place, this section will
include exact setup, build, test, and run commands for each component.

## Design principles

-   **Local-first:** Basic local server management should not depend on
    a cloud service.
-   **Clear responsibilities:** Keep UI, server-management logic, and
    remote connectivity separated.
-   **Safe operations:** Validate inputs, handle errors, and avoid
    destructive file operations without clear user intent.
-   **Secure by design:** Protect credentials, authenticate remote
    connections, and enforce permissions at the appropriate boundary.
-   **Compatibility-aware:** Treat Minecraft versions, Java
    requirements, loaders, and mod dependencies as explicit
    compatibility constraints.
-   **Incremental development:** Add and test one capability at a time
    rather than attempting a large rewrite all at once.
-   **Transparent status:** Document what is implemented, what is
    experimental, and what remains planned.

## Contributing

Tinker is under active development, and its architecture may change.
Before contributing a large feature, open an issue to discuss the
proposed behavior and design.

Useful contributions may include: - Bug reports with reproducible steps
and relevant logs. - Small, focused fixes with tests where practical. -
Documentation improvements. - Research into Minecraft version metadata,
loaders, world formats, or compatibility rules.

Please do not include passwords, access tokens, private server
addresses, or other sensitive information in issues or logs.

## License

A license has not yet been selected. Until a license is added to the
repository, do not assume that the source is available for unrestricted
reuse, redistribution, or commercial use.

## Project goals

Tinker is being built to make Minecraft server management easier while
giving the project a maintainable foundation for future features. The
immediate focus is learning, prototyping, and establishing a reliable
local management engine before expanding into modpack automation, world
maps, and remote access.
