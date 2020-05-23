pub mod common;
pub mod process;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
fn main(){
    // remote::api::test();
    //  process::flat_js::test();
     process::nested_js::test();
     // process::define::test();
}

// SET LIBZMQ_PREFIX=./lib

