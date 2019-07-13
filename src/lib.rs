#[macro_use]
extern crate log;
extern crate systemstat;

use std::{
    env, process,
    result::Result,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use jsonrpc_core::{types::error::Error, IoHandler, Params, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
#[allow(unused_imports)]
use jsonrpc_test as test;
use serde::Deserialize;
use snafu::{ensure, ResultExt};
use systemstat::{Platform, System};

//use crate::error::{I2CError, InvalidCoordinate, InvalidString, OledError};
/*
//define the Graphic struct for receiving draw commands
#[derive(Debug, Deserialize)]
pub struct Graphic {
    bytes: Vec<u8>,
    width: u32,
    height: u32,
    x_coord: i32,
    y_coord: i32,
}
*/

pub fn run() -> Result<(), Error> {
    info!("Starting up.");

    debug!("Creating handle for system statistics platform.");
    let sys = Arc::new(Mutex::new(System::new()));
    let sys_clone = Arc::clone(&sys);

    info!("Creating JSON-RPC I/O handler.");
    let mut io = IoHandler::default();

    // Returns the current CPU temperature in degrees Celsius.
    io.add_method("cpu_temp", move |_| {
        info!("Fetching CPU temperature.");
        let sys = sys_clone.lock().unwrap();
        let temp = match sys.cpu_temp() {
            Ok(temp_celcius) => temp_celcius.to_string(),
            // returns io error: struct std::io::Error
            Err(e) => format!("Error: {:?}", e),
        };

        Ok(Value::String(temp))
    });

    let sys_clone = Arc::clone(&sys);

    // ERR: gives wrong answer (ie. "off" when charging)
    // Returns whether AC power is plugged in.
    io.add_method("on_ac_power", move |_| {
        info!("Fetching AC power status.");
        let sys = sys_clone.lock().unwrap();
        let ac_on = match sys.on_ac_power() {
            Ok(ac) => {
                if ac {
                    "on".to_string()
                } else {
                    "off".to_string()
                }
            }
            Err(e) => format!("Error: {:?}", e),
        };

        Ok(Value::String(ac_on))
    });

    let sys_clone = Arc::clone(&sys);

    // Returns the system uptime.
    io.add_method("uptime", move |_| {
        info!("Fetching system uptime.");
        let sys = sys_clone.lock().unwrap();
        let uptime = match sys.uptime() {
            Ok(time) => format!("{:?}", time),
            Err(e) => format!("Error: {:?}", e),
        };

        Ok(Value::String(uptime))
    });

    let sys_clone = Arc::clone(&sys);

    // Returns a memory information object.
    io.add_method("memory", move |_| {
        info!("Fetching memory usage statistics.");
        let sys = sys_clone.lock().unwrap();
        match sys.memory() {
            Ok(m) => println!("{:?}", m),
            Err(e) => println!("Error: {:?}", e),
        };

        Ok(Value::String("success".to_string()))
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
