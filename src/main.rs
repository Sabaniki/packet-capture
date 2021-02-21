mod util;
mod interface;
mod handler;
mod packet;

#[macro_use]
extern crate log;

use std::env;
use util::app::get_arg;
use std::process::exit;
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let interface_name = get_arg().unwrap();

    // 引数からインターフェイスを選択
    let interface = interface::get_from_name(interface_name)
        .unwrap_or_else(|e| {
            error!("{}", e);
            exit(-1);
        });

    // データリンクのチャンネルを取得
    let (_tx, mut rx) = match datalink::channel(
        &interface,
        Default::default(),
    ) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create data-link channel {}", e)
    };

    loop {
        
    }
}
