extern crate minigrep;
use std::env;
use minigrep::Config;
use std::process;

fn main() {
	//chức năng env::args trả về một interator. Thay vì thu thập các giá trị ở đối số nhập vào rồi đưa vào 
	//Vector dể Config::new, bây giờ lấy quyền sở hữu của trả về từ env::args() để Config::new
	//trưc tiếp
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
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


