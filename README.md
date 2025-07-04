
# RCPU - Remote System Monitor API

[![Rust](https://img.shields.io/badge/Rust-1.72%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Web%20Framework-Axum-blue)](https://github.com/tokio-rs/axum)

A lightweight API server for monitoring system resources (CPU/RAM/Disk) built in Rust. Designed for remote visualization projects like ESP8266-based desktop displays.

## Features

- **Real-time CPU Load Monitoring**
  - Precise CPU usage calculation
  - Accurate kernel statistics using `/proc/stat`
- **Memory Monitoring**
  - RAM usage percentage calculation
  - Uses `/proc/meminfo` for accurate data
- **Disk Usage Monitoring**
  - Percentage-based usage
  - Byte-level usage (used/total)
- **RESTful API Endpoints**
  - Simple JSON responses
  - Built with Axum framework
- **Efficient Resource Usage**
  - Low memory footprint
  - Minimal CPU overhead

## Getting Started 🚀

### Prerequisites

- Rust 1.72+ ([Installation Guide](https://www.rust-lang.org/tools/install))
- Linux-based OS (for `/proc` filesystem access, `libc` usage)

### Installation

```bash
git clone https://github.com/qqweweqewe/rcpu.git
cd rcpu
./install.sh
```

### Usage

```bash
# Run the server manually
cargo run --release

# Or start the systemd service (not enabled by default)
sudo systemctl start rcpu
# Check if started
sudo systemctl status rcpu

# Test CPU monitoring
curl http://localhost:3000/cpu
# Sample response: {"cpu":42}
```

## API Reference 📖

### Endpoints

| Endpoint              | Method | Description                     | Response Format                     |
|-----------------------|--------|---------------------------------|-------------------------------------|
| `/cpu`                | GET    | Get current CPU load %          | `{"cpu": 42}`                       |
| `/ram`                | GET    | Get RAM usage %                 | `{"ram": 68}`                       |
| `/disk/percentage`    | GET    | Get disk usage %                | `{"percentage": 75}`                |
| `/disk/bytes`         | GET    | Get disk usage in bytes         | `{"used": 123456, "total": 987654}` |

## Technical Implementation 🔧

### CPU Monitoring
1. Read `/proc/stat` and parse CPU metrics
2. Calculate:
   - Total CPU time = sum of all states
   - Idle time = (idle + iowait) states
3. Take measurements at 100ms intervals
4. Compute percentage:
   ```
   usage = 100 - (Δidle_time / Δtotal_time) * 100
   ```

### RAM Monitoring
- Parses `/proc/meminfo`
- Calculations:
  ```
  usage = 100 - (MemAvailable * 100 / MemTotal)
  ```

### Disk Monitoring
- Uses `statvfs` system call
- Calculations:
  - `used`: `free_blocks * block_size`
  - `total`: `total_blocks * block_size`
- Supports:
  - Percentage: `100 - (free_blocks ** * 100 / total_blocks)`
  - Bytes: `(total_blocks * block_size, used_blocks * block_size)`

### Project Structure
```
src/
├── main.rs         # API server implementation
├── error.rs        # Custom error handling
├── info.rs         # System monitoring core
│   ├── cpu        # CPU module
│   ├── ram        # RAM module
│   └── disk       # Disk module
```

## Error Handling
- `404 Not Found` for invalid routes
- `500 Internal Server Error` for system errors
- Custom error messages for parsing failures

## Roadmap 🗺️

- [x] Core CPU monitoring
- [x] RAM usage implementation
- [x] Disk monitoring
- [x] Systemd service integration
- [ ] Docker container support
- [ ] ESP8266 client example
- [ ] Authentication support
