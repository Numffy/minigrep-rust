use std::env;
use minigrep:: Config;

fn main() {
    let argumentos: Vec<String> = env::args().collect();

    let config = Config::new(&argumentos);

    println!("{}", config.filename);
    println!("{}", config.query);

  
    minigrep::run(config);

    
}

