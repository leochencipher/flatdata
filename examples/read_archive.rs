#[macro_use]
extern crate flatdata;

use std::cell::RefCell;
use std::env;
use std::path;
use std::process;
use std::rc::Rc;

use flatdata::Archive;

mod graph;

fn main() {
    let mut args = env::args();
    let prog_name = args.next().unwrap();
    let path = match args.next() {
        Some(p) => p,
        None => {
            println!("Usage: {} <flatdata_archive>", prog_name);
            process::exit(1);
        }
    };

    let storage =
        Rc::new(RefCell::new(flatdata::FileResourceStorage::new(path::PathBuf::from(path))));
    let g = graph::Graph::open(storage);
    println!("is_open = {}", g.is_open());
    println!("{:?}", g);

    for v in g.vertices().iter() {
        println!("{:?}", v);
    }

    for e in g.edges().iter() {
        println!("{:?}", e);
    }
}
