# peach-stats

JSON-RPC wrapper around the [systemstat](https://crates.io/crates/systemstat) crate.

### JSON-API

| Method | Description | Returns |
| --- | --- |
| `battery_life` | |
| `block_device_statistics` | |
| `boot_time` | |
| `cpu_load` | |
| `cpu_temp` | | x _Implemented but fails_
| `on_ac_power` | | x _Implemented but gives incorrect result_
| `cpu_stats` | | x _Implemented_
| `cpu_stats_percent` | | x _Implemented_
| `load_average` | | x _Implemented_
| `memory` | | x _Implemented as println!_
| `mounts` | |
| `socket_stats` | |
| `uptime` | Returns system uptime in seconds | x _Implemented_

### Environment

The JSON-RPC HTTP server address and port can be configured with the `PEACH_STATS_SERVER` environment variable:

`export PEACH_STATS_SERVER=127.0.0.1:5000`

When not set, the value defaults to `127.0.0.1:5113`.

Logging is made available with `env_logger`:

`export RUST_LOG=info`

Other logging levels include `debug`, `warn` and `error`.

### Setup

Clone this repo:

`git clone https://github.com/peachcloud/peach-stats.git`

Move into the repo and compile a build release"

`cd peach-stats`
`cargo build`

Run the binary:

`./target/release/peach-stats`

### Example Usage

WIP.

### Licensing

AGPL-3.0

