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
	//hàm env::args sẽ trả về std::env::Args có đặt điểm (Trait) Iterator 
    pub fn new (mut args: std::env::Args) -> Result<Config, &'static str> {
		//giá trị đầu tiên của args mặc định chứa tên của 
		//chương trình không phải là thứ chúng ta nhập vào
		//Bỏ qua nó bằng cách gọi next() và không làm gì với
		//giá trị trả lại
		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Không nhận được chuỗi query"),
		};

		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Không nhận được chuỗi filename"),
		};

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
    
	contents.lines()
		.filter(|line| line.contains(query))
		.collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
	let query = query.to_lowercase();
	contents.lines()
		.filter(|line| line.to_lowercase().contains(&query))
		.collect()

	// let mut results = Vec::new();
	// for line in contents.lines() {
	// 	//Vi query hien tai la String ma contains() su dung str 
	// 	// nen can them dau &
	//     if line.to_lowercase().contains(&query) {
	// 	    results.push(line);
	// 	}
	// }
	// results
}



