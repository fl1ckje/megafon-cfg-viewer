use std::iter::Peekable;
use std::str::Lines;

pub struct LineScanner<'a> {
    iter: Peekable<Lines<'a>>,
}

impl<'a> LineScanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.lines().peekable(),
        }
    }

    pub fn next_line(&mut self) -> Option<&'a str> {
        while let Some(line) = self.iter.next() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
        None
    }

    pub fn peek_line(&mut self) -> Option<&'a str> {
        while let Some(line) = self.iter.peek() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                self.iter.next();
            } else {
                return Some(trimmed);
            }
        }
        None
    }

    pub fn parse_kv(line: &str) -> Option<(&str, &str)> {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            Some((parts[0].trim(), parts[1].trim()))
        } else {
            None
        }
    }

    pub fn clean_string(val: &str) -> String {
        val.trim_matches('"').to_string()
    }

    pub fn is_closing_tag(line: &str, section_name: &str) -> bool {
        line == format!("[#{}]", section_name)
    }

    pub fn get_section_name(line: &str) -> Option<&str> {
        if line.starts_with('[') && !line.starts_with("[#") && line.ends_with(']') {
            Some(line.trim_matches(|c| c == '[' || c == ']'))
        } else {
            None
        }
    }
}
