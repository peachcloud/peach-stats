# peach-stats

JSON-RPC wrapper around the [probes](https://crates.io/crates/probes) and [systemstat](https://crates.io/crates/systemstat) crates.

### JSON-API

| Method | Description | Returns |
| --- | --- | --- |
| `cpu_stats` | | |
| `cpu_stats_percent` | | |
| `disk_usage` | | |
| `load_average` | | |
| `mem_stats` | | |
| `uptime` | Returns system uptime in seconds & nanoseconds | |

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

