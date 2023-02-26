pub mod ports;
pub mod packet;
pub mod laminar_helper;

use std::io::{stdin};
use std::thread;
use std::time::Instant;
use color_eyre::Result;
const SERVER: &str = "127.0.0.1:12351";

