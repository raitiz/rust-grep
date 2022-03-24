use std::env;
use std::error::Error;
use std::fs;

pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(params.filename)?;
    let results = if params.case_sensitive {
        search(&params.query, &contents)
    } else {
        search_case_insesitive(&params.query, &contents)
    };
    for line in results {
        println!("{}", line)
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
pub fn search_case_insesitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

pub struct Params {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Params {
    pub fn new(args: &Vec<String>) -> Result<Params, &'static str> {
        if args.len() < 3 {
            return Err("Not enaugh arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var(" ").is_err();
        Ok(Params {
            query,
            filename,
            case_sensitive,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.
Pick tree.";
        assert_eq!(vec!("safe, fast, productive."), search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";
        let res = vec!["Rust:", "Trust me."];
        assert_eq!(res, search_case_insesitive(query, contents));
    }
}
