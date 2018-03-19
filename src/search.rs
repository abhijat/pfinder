pub struct Search {
    pub inverted: bool,
    pub case_sensitive: bool,
    pub term: String,
}

impl Search {
    pub fn new(inverted: bool, case_sensitive: bool, term: &str) -> Self {
        println!("case_sensitive is {}", case_sensitive);
        Search { inverted, case_sensitive, term: term.to_owned() }
    }
}