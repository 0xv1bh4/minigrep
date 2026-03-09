pub fn search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    _contents.lines().filter(|line| line.contains(_query)).collect()
}

pub fn search_case_insensitive<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    _contents.lines().filter(|line| line.to_lowercase().contains(_query.to_lowercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}