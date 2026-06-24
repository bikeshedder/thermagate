# Contributing to Thermagate

This document collects practical hints for working on Thermagate. For what the
project *is* and how to install/configure a release build, see the
[README](./README.md). For building release/cross-compiled binaries, see the
[Building from source](./README.md#building-from-source) section there.

## Getting a development environment

You have two options.

### Option A — Dev Container (recommended)

The repository ships a [Dev Container](https://containers.dev/) definition in
[`.devcontainer/`](./.devcontainer/). It provides a ready-to-go toolchain
(Rust, Node.js, `cross`, Docker-in-Docker) so you don't have to install
anything on your machine beyond Docker and an editor that understands the spec.

- **VS Code**: install the *Dev Containers* extension, open the repo, and
  choose *Reopen in Container*.
- **Other editors / CLI**: Dev Containers are an
  [open specification](https://containers.dev/), not a VS Code-only feature.
  You can build and enter the same environment with the
  [`devcontainer` CLI](https://github.com/devcontainers/cli):

  ```bash
  npm install -g @devcontainers/cli
  devcontainer up --workspace-folder .
  devcontainer exec --workspace-folder . bash
  ```

If you'd rather not use a container at all, use Option B — nothing in the
project *requires* the dev container.

### Option B — Local toolchain

1. Install [Rust](https://www.rust-lang.org/learn/get-started) (see
   `rust-version` in [`Cargo.toml`](./Cargo.toml) for the minimum).
2. Install [Node.js](https://nodejs.org/) 22 LTS or newer.
3. Build the frontend:

   ```bash
   cd frontend
   npm ci
   npm run build
   ```

4. Build/run the backend from the repo root:

   ```bash
   cargo run            # run directly
   cargo build --release # or produce an optimized binary
   ```

The Rust binary embeds the built frontend assets, so build the frontend before
the backend if you want the web UI to reflect your changes.

## CAN bus interface

Thermagate talks to the heat pump over a SocketCAN interface called `can0`.
You need such an interface available wherever the application runs — real CAN
hardware on the bus, or a virtual `vcan` interface for development without
hardware.

> The real heat pump CAN bus runs at a **bitrate of 20000**. Use that when
> configuring physical hardware; `vcan` interfaces are bitrate-less.

### Virtual CAN (no hardware, for development)

```bash
sudo modprobe vcan
sudo ip link add dev can0 type vcan
sudo ip link set can0 up
```

`vcan` is a host kernel module — load it on the host, not inside a container.

### Real CAN hardware

```bash
ip link show can0                                 # confirm it exists
sudo ip link set can0 type can bitrate 20000      # heat pump bus bitrate
sudo ip link set can0 up
```

For a Raspberry Pi + Waveshare CAN HAT setup, see
[Raspberry PI 4 configuration](./README.md#raspberry-pi-4-configuration) in the
README.

### Using `can0` from inside the Dev Container

The dev container's `app` service runs with `network_mode: host` (see
[`.devcontainer/compose.yml`](./.devcontainer/compose.yml)), so it shares the
host's network namespace and every host interface — including `can0` — is
visible inside the container with no extra steps. Configure `can0` on the host
as described above; nothing extra is needed in the container.

Verify inside the container:

```bash
ip -details link show can0
```

You should see the same `can0` that exists on the host.

### CAN troubleshooting

| Symptom | Cause / fix |
| --- | --- |
| `can0` not found (in container or locally) | The interface doesn't exist on the host. Create/connect it (see above). |
| `can0` exists but is `DOWN` | Bring it up: `sudo ip link set can0 up`. |
| Real CAN sees no traffic | Bitrate mismatch — the heat pump bus is `20000`; set it before bringing the interface up. |

## MQTT broker

Thermagate is most useful paired with an MQTT broker. The Dev Container brings
up a [Mosquitto](https://mosquitto.org/) broker as a second compose service
(`mosquitto`) so you don't have to run one yourself.

Point Thermagate's `[mqtt]` config at it:

```toml
[mqtt]
host = "mosquitto"   # or 172.30.0.2
port = 1883
```

A few things worth knowing about how it's wired up:

- The broker is **not published to the host** — there is no `ports:` mapping.
  It listens only on a private bridge network and is reachable from the
  `app` container (and, because the host routes to the bridge, from the host
  itself), but not from the LAN.
- Because the `app` service uses `network_mode: host`, it cannot also join the
  bridge network, so Docker/Podman **service-name DNS does not work** from it.
  The broker is therefore pinned to a static IP (`172.30.0.2`) in
  [`compose.yml`](./.devcontainer/compose.yml), and the `app` service maps the
  name `mosquitto` to that IP via `extra_hosts`. That's why both `mosquitto`
  and `172.30.0.2` work as the host.
- If `172.30.0.0/24` clashes with an existing network on your machine, change
  the subnet and the static IP in `compose.yml` (and the `extra_hosts` entry).
- The broker allows anonymous access and uses no TLS — **development only**,
  configured in [`mosquitto.conf`](./.devcontainer/mosquitto.conf).

## Configuration

The binary prints its default configuration with `cargo run -- default-config`
(or `thermagate default-config`); redirect it to a `config.toml` and edit. The
defaults also live in
[`src/config/default.toml`](./src/config/default.toml). See
[Configuration file](./README.md#configuration-file) in the README for details.
