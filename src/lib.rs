pub fn search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in _contents.lines() {
        if line.contains(_query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = _query.to_lowercase();
    for line in _contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
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