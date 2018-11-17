extern crate minigrep;
use std::env;
use minigrep::Config;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
	
	let config = Config::new(&args).unwrap_or_else(|err|{
	
	    //eprintln! chi in loi tren dong lenh, khong in 
		//loi trong file output.txt khi chay cargo run > output.txt
		eprintln!("Problem parsing arguments: {}", err);
		
		//exit code cua window la 256, linux la 1,
        //Hien thong bao: "process didn't exit successfully"
		//Chua biet cach xu ly ra sao
		process::exit(256);
	});

    println!("Searching for {}", config.query);
    println!("In file {}:
", config.filename);
	
	//run(config)
	if let Err(e) = minigrep::run(config) {
	    eprintln!("Application error: {}", e);
		process::exit(256);
	}
	
		
}


