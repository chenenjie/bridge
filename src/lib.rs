#![crate_name = "bridge"]
#![crate_type = "lib"]

#[macro_use]
extern crate error_chain;
extern crate tokio_core;
extern crate tokio_io;
extern crate futures;


mod bridge;
mod result;

pub use bridge::run as bridge_run;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
