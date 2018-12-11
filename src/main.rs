use getopts::Options;
use std::env;
use std::io;

mod year2018;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.reqopt("y", "year", "year to run", "YEAR");
    opts.reqopt("d", "day", "day to run", "DAY");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let year = match matches.opt_str("y") {
        Some(y) => y,
        None => "".to_string(),
    };
    let instance = match matches.opt_str("d") {
        Some(d) => d,
        None => "".to_string(),
    };

    match year.as_ref() {
        "2018" => match instance.as_ref() {
            "1-1" => year2018::d01p1::main(),
            "1-2" => year2018::d01p2::main(),
            "2-1" => year2018::d02p1::main(),
            "2-2" => year2018::d02p2::main(),
            "3-1" => year2018::d03p1::main(),
            "3-2" => year2018::d03p2::main(),
            "4-1" => year2018::d04p1::main(),
            "4-2" => year2018::d04p2::main(),
            "5-1" => year2018::d05p1::main(),
            "5-2" => year2018::d05p2::main(),
            "6-1" => year2018::d06p1::main(),
            "6-2" => year2018::d06p2::main(),
            "7-1" => year2018::d07p1::main(),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Day not found")),
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "Year not found")),
    }
}
