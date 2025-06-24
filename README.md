# RCPU - Remote System Monitor API
## Single endpoint branch

[![Rust](https://img.shields.io/badge/Rust-1.72%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Web%20Framework-Axum-blue)](https://github.com/tokio-rs/axum)

A lightweight API server for monitoring system resources (CPU/RAM) built in Rust. Designed for remote visualization projects like ESP8266-based desktop displays.

## Features

- **Real-time CPU Load Monitoring**
  - Precise CPU usage calculation
  - Accurate kernel statistics using `/proc/stat`
- **Memory Monitoring**
  - Multi-parameter monitoring (coming soon)
- **Convenient API interaction**
  - Simple JSON responses
  - Verbosity controlled with query parameters
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
curl http://localhost:3000/stats
# Sample response: 
# {
#   "cpu":"43",
#   "ram":"68",
#   "err":null
# }
```

## API Reference 📖

### Endpoints

| Endpoint | Method | Description                | Response Format        |
|----------|--------|----------------------------|------------------------|
| `/stats` | GET    | Get current CPU load %     | `{"cpu": "42", "ram": "36", "err": null}`      |

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

### RAM Polling algorithm
1. Read `/proc/meminfo` and parse RAM data
2. Compute percentage:
   ```
   usage = 100 - ( free / used ) * 100
   ```

### Project Structure
```
src/
├── main.rs         # API server implementation
├── info.rs         # System monitoring core
│   ├── cpu::get_cpu_load()  # CPU load calculation
│   └── ram::get_busy()      # RAM module
```

## Roadmap 🗺️

- [x] Core CPU monitoring
- [x] RAM usage implementation
- [ ] Systemd service integration
- [ ] Docker container support
- [ ] ESP8266 client example
- [ ] Authentication support
