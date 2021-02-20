mod util;
mod interface;

#[macro_use]
extern crate log;

use std::env;
use util::app::get_arg;
use std::process::exit;


fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let interface_name = get_arg().unwrap_or_else(|e| {
        error!("{}", e);
        "".to_string()
    });

    let interface = interface::get_from_name(interface_name)
        .unwrap_or_else(|e|{
            error!("{}", e);
            exit(-1);
        });
    print!("{:?}", interface);
}
