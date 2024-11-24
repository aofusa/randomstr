
use clap::Parser;
use rand::Rng;
use randomstr::randomstr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    length: usize,

    #[arg(short, long, default_value_t = 1)]
    number_of_strings: usize,

    #[arg(long)]
    range: Option<usize>
}

fn main() {
    let args = Args::parse();

    if args.range.is_some() {
        let mut rng = rand::thread_rng();
        for _ in 0..args.number_of_strings {
            let m = args.range.unwrap();
            let l = rng.gen_range(m..args.length+1);
            let s = randomstr(l);
            println!("{}", s);
        }
    } else {
        for _ in 0..args.number_of_strings {
            let s = randomstr(args.length);
            println!("{}", s);
        }
    }
}
