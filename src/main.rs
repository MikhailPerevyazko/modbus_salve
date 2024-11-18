mod config;
mod connect;

use config::parse_to_json_format;
use connect::{connect_tcp, connect_udp};
fn main() {
    let vec_json_config = parse_to_json_format();
    connect_tcp(vec_json_config.to_owned());
    connect_udp(vec_json_config.to_owned());
}
