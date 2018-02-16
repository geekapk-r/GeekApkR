extern crate rust-ini;

use rust-ini::*;

const INIFILE :&'static str = "./geekapk.ini";

pub struct ServerConfig {

}

pub fn parse_cfg() -> ServerConfig {
    ServerConfig {}
}
