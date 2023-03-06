use rand::Rng;
use std::{env, process};

fn usage() -> ! {
    eprintln!("usage: gen [n: number of node]");
    process::exit(1);
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        usage();
    }

    let mut rng = rand::thread_rng();
    let n = args[1].parse::<usize>().unwrap();
    println!("{}", n);
    for _ in 0..n {
        let x = rng.gen_range(0, 1000);
        let y = rng.gen_range(0, 1000);
        println!("{} {}", x, y);
    }
}
