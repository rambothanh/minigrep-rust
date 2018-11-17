use std::error::Error;
use std::io::prelude::*;
use std::env;
use std::fs::File;

pub struct Config {
    pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
	    if args.len() < 3 {
		    //Se thong bao loi 
			//va ket thuc chuong trinh ngay lap luc
		    return Err("not enough arguments");
			
		}
	    let query    = args[1].clone();
	    let filename = args[2].clone();
		
		//neu bien muoi truong CASE_INSENSITIVE duoc set thi case_sensitive = true
		//neu bien muoi truong CASE_INSENSITIVE khong duoc set thi
		//case_sensitive = is_err() = false
		//Set evn trong window bi loi, chua biet cach khac phuc
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
		
	    Ok(Config {query, filename, case_sensitive})
	}
}

pub fn run(config: Config) -> Result<(),Box <Error> >{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

	//mac dinh search_case_insensitive khong phan biet chu hoa va
	//chu thuong
	//Set evn trong window bi loi, chua biet cach khac phuc
    let results = if config.case_sensitive {
	    search_case_insensitive(&config.query, &contents)
	} else {
	    search(&config.query, &contents)
	};
	
	for line in results {
	    println!("{}",line);
	}
	
	Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
	
	#[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    
	let mut results = Vec::new();
	for line in contents.lines() {
	    if line.contains(query) {
		    results.push(line);
		}
	}
	results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
	let mut results = Vec::new();
	for line in contents.lines() {
		//Vi query hien tai la String ma contains() su dung str 
		// nen can them dau &
	    if line.to_lowercase().contains(&query) {
		    results.push(line);
		}
	}
	results
}



