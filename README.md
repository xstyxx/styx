# styx

Collaborative encrypted terminal sharing on an infinite canvas.

**Live at [xstyx.io](https://xstyx.io)**

## Install

```sh
curl -sSf https://xstyx.io/get | sh
```

Then run `styx` to start a session. Share the link with anyone.

### Quick run (no install)

```sh
curl -sSf https://xstyx.io/get | sh -s run
```

### Download only

```sh
curl -sSf https://xstyx.io/get | sh -s download
```

## Features

- **End-to-end encrypted** — AES-128-CTR with Argon2id key derivation
- **Infinite canvas** — arrange multiple terminals freely, zoom and pan
- **Real-time collaboration** — live cursors, chat, and presence
- **Cross-platform** — Linux, macOS, FreeBSD, Windows
- **Lightweight** — single static binary, no dependencies
- **Self-hosted** — run your own relay server

## Platforms

| OS | Architectures |
|---|---|
| Linux | x86_64, aarch64, armv7 |
| macOS | x86_64, aarch64 (Apple Silicon) |
| FreeBSD | x86_64 |
| Windows | x86_64, i686 |

## Self-hosting

See [SELF_HOSTING.md](SELF_HOSTING.md) for instructions on running your own styx relay server.

## Development

Requirements: Rust, Node.js, Redis

```sh
# Frontend
npm install
npm run dev

# Server
cargo run --bin styx-server

# Client
cargo run --bin styx
```

## License

MIT
