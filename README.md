# peach-stats

System statistics microservice module for PeachCloud. Provides a JSON-RPC wrapper around the [probes](https://crates.io/crates/probes) and [systemstat](https://crates.io/crates/systemstat) crates.

### JSON-API

| Method | Description | Returns |
| --- | --- | --- |
| `cpu_stats` | CPU statistics | `user`, `system`, `nice`, `idle` |
| `cpu_stats_percent` | CPU statistics as percentages | `user`, `system`, `nice`, `idle` |
| `disk_usage` | Disk usage statistics (array of disks) | `filesystem`, `one_k_blocks`, `one_k_blocks_used`, `one_k_blocks_free`, `used_percentage`, `mountpoint` |
| `load_average` | Load average statistics | `one`, `five`, `fifteen` |
| `mem_stats` | Memory statistics | `total`, `free`, `used` |
| `uptime` | System uptime | `secs`, `nanos` |

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

**Get CPU Statistics**

With microservice running, open a second terminal window and use `curl` to call server methods:

`curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "cpu_stats", "id":1 }' 127.0.0.1:5113`

Server responds with:

`{"jsonrpc":"2.0","result":"{\"user\":4661083,\"system\":1240371,\"idle\":326838290,\"nice\":0}","id":1}`

**Get System Uptime**

With microservice running, open a second terminal window and use `curl` to call server methods:

`curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "uptime", "id":1 }' 127.0.0.1:5113`

Server responds with:

`{"jsonrpc":"2.0","result":"{\"secs\":840968,\"nanos\":0}","id":1}`

### Licensing

AGPL-3.0

