extern crate clap;
mod iterator;

use iterator::{AbstractIterator, VecIterator, BHIterator};
use clap::{App, Arg};
use std::process;
use std::collections::BinaryHeap;


fn add_iter<V: AbstractIterator<i32>>(mut iter: V) -> i32 {
    let mut sum = 0;
    loop {
        match iter.next() {
            Some(v) => sum += v,
            None => break
        };
    }
    sum
}

fn display_iter<V: AbstractIterator<i32>>(mut iter: V) {
    loop {
        match iter.next() {
            Some(v) => println!("{}", v),
            None => break
        }
    }
}

fn main() {
    let matches = App::new("Iterator Design Pattern")
        .version("0.1")
        .author("Bărbuț-Dică Sami aka WarriorsSami")
        .about("Simple use case for Iterator Design Pattern which involves \
                two different kinds of iterators (designed for a list and, respectively, for a heap)")
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("set data structure to list"))
        .arg(Arg::with_name("heap")
            .short("h")
            .long("heap")
            .help("set data structure to heap"))
        .arg(Arg::with_name("data")
            .multiple(true)
            .help("insert mock data to fill the data structure"))
        .get_matches();

    if !matches.is_present("data") {
        println!("Please insert some mock data!");
        process::exit(1)
    }

    let non_parsed: Vec<&str> = matches.values_of("data").unwrap().collect();
    let mut data: Vec<i32> = Vec::new();

    for d in non_parsed {
        match d.parse::<i32>() {
            Ok(v) => data.push(v),
            Err(e) => {
                println!("Unable to parse given data: {}", e);
                process::exit(1);
            }
        };
    }

    if matches.is_present("list") {
        let iter = VecIterator::new(&data);
        //println!("The result of addition is {}", add_iter(iter));
        display_iter(iter)
    } else if matches.is_present("heap") {
        let mut heap = BinaryHeap::new();
        for d in data {
            heap.push(d);
        }
        let iter = BHIterator::new(heap);
        //println!("The result of addition is {}", add_iter(iter));
        display_iter(iter)
    } else {
        println!("Please provide a suitable data structure!");
        process::exit(1);
    }
}
