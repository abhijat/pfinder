use search::Search;

#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    pub name: String,
    pub command_line: String,
}

impl Process {
    pub fn new(_name: &str, _command_line: &str) -> Self {
        Process {
            name: _name.to_owned(),
            command_line: _command_line.to_owned(),
        }
    }

    pub fn matches(&self, search: &Search) -> bool {
        let mut name = self.name.to_owned();
        let mut cmd = self.command_line.to_owned();
        let mut term = search.term.clone();

        if !search.case_sensitive {
            name = name.to_lowercase();
            cmd = cmd.to_lowercase();
            term = term.to_lowercase();
        }

        if search.inverted {
            !name.contains(&term) && !cmd.contains(&term)
        } else {
            name.contains(&term) || cmd.contains(&term)
        }
    }
}