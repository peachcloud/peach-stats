#[macro_use]
extern crate log;

mod error;
mod stats;

use std::{env, result::Result};

use jsonrpc_core::{types::error::Error, IoHandler, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
#[allow(unused_imports)]
use jsonrpc_test as test;
use probes::*;

use crate::error::{BoxError, StatError};

pub fn run() -> Result<(), BoxError> {
    info!("Starting up.");

    info!("Creating JSON-RPC I/O handler.");
    let mut io = IoHandler::default();

    // current cpu stats
    io.add_method("cpu_stats", move |_| {
        info!("Fetching CPU statistics.");
        match stats::cpu_stats() {
            Ok(stats) => Ok(Value::String(stats)),
            Err(_) => Err(Error::from(StatError::GetCpuStat)),
        }
    });

    // current cpu stats as percentages
    io.add_method("cpu_stats_percent", move |_| {
        info!("Fetching CPU statistics as percentages.");
        match stats::cpu_stats_percent() {
            Ok(stats) => Ok(Value::String(stats)),
            Err(_) => Err(Error::from(StatError::GetCpuStat)),
        }
    });

    // disk usage load stats
    io.add_method("disk_usage", move |_| {
        info!("Fetching CPU temperature.");
        let disk_usages = disk_usage::read();
        if let Ok(disk) = disk_usages {
            println!("{:?}", disk);
        }

        Ok(Value::String("success".to_string()))
    });

    // current load average of the system (one, five, fifteen)
    io.add_method("load_average", move |_| {
        info!("Fetching system load average statistics.");
        match stats::load_average() {
            Ok(avgs) => Ok(Value::String(avgs)),
            Err(_) => Err(Error::from(StatError::GetLoadAvg)),
        }
    });

    let http_server = env::var("PEACH_OLED_STATS").unwrap_or_else(|_| "127.0.0.1:5113".to_string());

    info!("Starting JSON-RPC server on {}.", http_server);
    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(
            &http_server
                .parse()
                .expect("Invalid HTTP address and port combination"),
        )
        .expect("Unable to start RPC server");

    info!("Listening for requests.");
    server.wait();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use jsonrpc_core::ErrorCode;
    use std::io::Error as IoError;
    use std::io::ErrorKind;

    // test to ensure correct success response
    #[test]
    fn rpc_success() {
        let rpc = {
            let mut io = IoHandler::new();
            io.add_method("rpc_success_response", |_| {
                Ok(Value::String("success".into()))
            });
            test::Rpc::from(io)
        };

        assert_eq!(rpc.request("rpc_success_response", &()), r#""success""#);
    }
}
