use minigrep::Params;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Params::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", conf.query);
    println!("In file {} \n", conf.filename);

    if let Err(e) = minigrep::run(conf) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
