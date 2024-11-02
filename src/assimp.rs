#![allow(clippy::all)]
// I had to do this because of u128s, apparently its not really a problem anymore though?
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
