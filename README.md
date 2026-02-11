# Uptix Agent (Rust) 🚀

A lightweight, high-performance monitoring agent for the [Uptix](https://github.com/xavdp-pro/uptix-oss) ecosystem. 
Built with **Rust** for maximum efficiency, safety, and zero dependencies.

## Features
- **System Metrics**: Real-time CPU usage, RAM consumption, and Disk status.
- **Service Uptime**: Monitor multiple websites and services via HTTP/HTTPS.
- **Fast & Light**: Low CPU/RAM footprint, compiled to a single static binary.
- **Real-time**: WebSocket (Socket.io) communication for instant updates.

## Configuration
Edit `sites.json` in the agent directory:
```json
[
  { "url": "https://google.com" },
  { "url": "https://your-app.com" }
]
```

## Compilation
```bash
cargo build --release
```
The binary will be available at `./target/release/uptix-agent`.

## License
MIT
