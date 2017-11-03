use regex::Regex;

#[derive(Debug, Clone)]
struct Lexer {
    tokens: Vec<String>,
    pos: usize,
}

impl Lexer {
    fn next(&mut self) -> Option<String> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
            Some(self.tokens[self.pos - 1].to_string())
        } else {
            None
        }
    }

    fn peek(&self) -> Option<String> {
        if self.pos < self.tokens.len() {
            Some(self.tokens[self.pos].to_string())
        } else {
            None
        }
    }
}

fn tokenize(str: String) -> Vec<String> {
    let mut results = vec![];
    let re = Regex::new(r###"[\s,]*([()']|"(?:\\.|[^\\"])*"|#.*|[^\s,()'"]*"###).unwrap();
    for cap in re.captures_iter(&str) {
        let group = match cap.get(1) {
            Some(mtch) => mtch.as_str(),
            None => "",
        };
        if group == "" { break; }
        if group.starts_with("#") { continue; }
        results.push(group.to_owned());
    }
    results
}
