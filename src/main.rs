extern crate clap;
use std::thread;
use std::alloc;
use std::ptr;
use clap::{Arg, App};
fn main() {
    let matches = App::new("Resource killer")
        .version("1.0")
        .author("Pagwin <pagwin.fedora@protonmail.com>")
        .about("Uses resources")
        .arg(Arg::with_name("thread-count")
            .short("c")
            .long("thread-count")
            .value_name("COUNT")
            .help("sets the number of threads that get maxed out")
            .takes_value(true))
        .arg(Arg::with_name("ram-usage")
            .short("r")
            .long("ram-usage")
            .value_name("USAGE")
            .help("sets the amount of ram used")
            .takes_value(true))
        .get_matches();
    let thread_count:u16 = matches.value_of("thread-count").unwrap_or("1").parse().unwrap_or_else(|_|{
        eprintln!("Invalid value provided for thread count defaulting to 1");
        1
    });
    let ram_usage:usize = ram_parse(matches.value_of("ram-usage").unwrap_or("0"));
    let mut threads = Vec::new();
    for _ in 0..thread_count-1 {
        threads.push(thread::spawn(|| {
            loop {};
        }));
    }
    let mem;
    unsafe{
        mem = alloc::alloc(alloc::Layout::from_size_align(ram_usage, 1).unwrap());
        ptr::write_bytes(mem, 0xaa, ram_usage);
    }
    loop{}
    //for thread in threads {
    //    thread.join().unwrap();
    //}
}
fn ram_parse(usage_str:&str) -> usize {
    let mut internal = String::from(usage_str);
    let mut multiplier:usize = match internal.pop().unwrap_or(' ') {
        //pretty sure rust supports scientific notation which would make this more concise but I
        //personally find this more entertaining to look at
        'k' | 'K' => 1000,
        'm' | 'M' => 1000000,
        'g' | 'G' => 1000000000,
        't' | 'T' => 1000000000000,
        'p' | 'P' => 1000000000000000,
        _ => 1
    };
    let mut value:f64 = internal.parse().unwrap_or_else(|_| {
        eprintln!("Invalid value provided for ram usage defaulting to 0");
        0.0
    });
    while value % 1.0 != value && multiplier >=10 {
        value *= 10.0;
        multiplier /= 10;
    };
    value as usize * multiplier
}
