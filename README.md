# Uptix Agent (Rust) 🚀

Ultra-lightweight monitoring agent for Uptix.

## Quick Start (Docker)
```bash
docker run -d \
  -e UPTIX_HUB_URL="http://your-hub-ip:3001" \
  -e UPTIX_SERVER_NAME="my-web-server" \
  -e UPTIX_MONITOR_SITES="https://google.com" \
  uptix-agent
```

## Features
- Zero Configuration (uses defaults).
- Tiny footprint.
- Environment variable driven.
