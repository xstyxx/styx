<p align="center">
  <img src="static/favicon.svg" width="80" />
</p>

<h1 align="center">styx</h1>

<p align="center">
  <strong>Encrypted collaborative terminals on an infinite canvas.</strong>
</p>

<p align="center">
  <a href="https://xstyx.io">Website</a> &middot;
  <a href="https://github.com/xstyxx/styx/releases">Releases</a>
</p>

---

## Install

```sh
curl -sSf https://xstyx.io/get | sh
```

Run `styx` to start a session. Share the link — anyone with the URL can join.

```sh
$ styx

  styx v0.4.1

  > Link:  https://xstyx.io/s/abc123#secret
  > Shell: /bin/bash
```

### One-liner (no install)

```sh
curl -sSf https://xstyx.io/get | sh -s run
```

## Features

- **End-to-end encrypted** — AES-128-CTR + Argon2id. Server never sees plaintext.
- **Infinite canvas** — multiple terminals, drag to arrange, zoom and pan.
- **Real-time** — live cursors, presence indicators, built-in chat.
- **Cross-platform** — single static binary. No runtime dependencies.
- **Self-hosted** — bring your own relay server.

## Supported Platforms

| OS | Architectures |
|---|---|
| Linux | x86_64, aarch64, armv7, arm |
| macOS | x86_64, Apple Silicon |
| FreeBSD | x86_64 |
| Windows | x86_64, i686 |

## How It Works

1. Client generates a random encryption key and derives an AES key via Argon2id
2. Terminal I/O is encrypted client-side before transmission
3. The relay server only sees ciphertext — cannot read terminal content
4. The key is in the URL fragment (`#`), which is never sent to the server

## Development

Requires: Rust 1.75+, Node.js 18+, Redis

```sh
npm install          # frontend deps
npm run dev          # start dev server (SvelteKit)

cargo run -p styx-server   # relay server
cargo run -p styx          # client binary
```

## Reference

https://github.com/ekzhang/sshx
