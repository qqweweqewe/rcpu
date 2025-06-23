# RCPU - Remote System Monitor API

[![Rust](https://img.shields.io/badge/Rust-1.72%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Web%20Framework-Axum-blue)](https://github.com/tokio-rs/axum)

A lightweight API server for monitoring system resources (CPU/RAM) built in Rust. Designed for remote visualization projects like ESP8266-based desktop displays.

## Features

- **Real-time CPU Load Monitoring**
  - Precise CPU usage calculation
  - Accurate kernel statistics using `/proc/stat`
- **Memory Monitoring** (Coming Soon)
- **RESTful API Endpoints**
  - Simple JSON responses
  - Built with Axum framework
- **Efficient Resource Usage**
  - Low memory footprint
  - Minimal CPU overhead

## Getting Started 🚀

### Prerequisites

- Rust 1.72+ ([Installation Guide](https://www.rust-lang.org/tools/install))
- Linux-based OS (for `/proc/stat` access)

### Installation

```bash
git clone https://github.com/qqweweqewe/rcpu.git
cd rcpu
cargo build --release
```

### Usage

```bash
# Run the server
cargo run --release

# Test CPU monitoring (terminal mode)
curl http://localhost:3000/cpu
# Sample response: {"message":"42"}

# Test RAM endpoint (WIP)
curl http://localhost:3000/ram
```

## API Reference 📖

### Endpoints

| Endpoint | Method | Description                | Response Format        |
|----------|--------|----------------------------|------------------------|
| `/cpu`   | GET    | Get current CPU load %     | `{"message": "42"}`    |
| `/ram`   | GET    | Get RAM usage (WIP)        | `{"message": "WIP"}`   |

## Technical Implementation 🔧

### CPU Monitoring Algorithm
1. Read `/proc/stat` and parse CPU metrics
2. Calculate:
   - Total CPU time = sum of all states
   - Idle time = (idle + iowait) states
3. Take measurements at 1-second intervals
4. Compute percentage:
   ```
   usage = 100 - (Δidle_time / Δtotal_time) * 100
   ```

### Project Structure
```
src/
├── main.rs         # API server implementation
├── info.rs         # System monitoring core
│   ├── cpu::get_cpu_load()  # CPU load calculation
│   └── ram          # RAM module (WIP)
```

## Roadmap 🗺️

- [x] Core CPU monitoring
- [ ] RAM usage and other info implementation
- [ ] Systemd service integration
- [ ] Docker container support
- [ ] ESP8266 client example
- [ ] Authentication support
