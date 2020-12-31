use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CommandType {
    A_COMMAND,
    C_COMMAND,
    L_COMMAND,
}

#[derive(Debug)]
pub struct Parser {
    reader: BufReader<File>,
    current: String,
    next: String,
    is_eof: Option<bool>,
}

impl Parser {
    pub fn new(filename: &str) -> Parser {
        let f = File::open(filename).expect("file not found");
        Self{
            reader: BufReader::new(f),
            current: String::new(),
            next: String::new(),
            is_eof: None,
        }
    }

    pub fn has_more_content(&self) -> bool {
        match self.is_eof {
            Some(x) => !x,
            None => true,
        }
    }

    pub fn advance(&mut self) {
        self.current = self.next.clone();
        self.next.clear();
        let num_bytes = self.reader.read_line(&mut self.next);
        self.is_eof = Some(num_bytes.expect("something occurs") == 0);
        self.next = self.next.split("//").next().unwrap().to_string();
        self.next = self.next.replace(" ", "");
        self.next = self.next.trim().to_string();
        while self.next.len() == 0 {
            self.next.clear();
            let num_bytes = self.reader.read_line(&mut self.next);
            self.is_eof = Some(num_bytes.expect("something occurs") == 0);
            self.next = self.next.split("//").next().unwrap().to_string();
            self.next = self.next.replace(" ", "");
            self.next = self.next.trim().to_string();
            if let Some(x) = self.is_eof {
                if x { break; }
            }
        }
        if self.current.is_empty() {
            self.advance();
        }
    }

    pub fn command_type(&self) -> CommandType {
        if self.current.starts_with("@") {
            CommandType::A_COMMAND
        } else if self.current.starts_with("(") {
            CommandType::L_COMMAND
        } else {
            CommandType::C_COMMAND
        }
    }

    pub fn symbol(&self) -> String {
        match self.command_type() {
            CommandType::A_COMMAND => {
                self.current.replace("@", "")
            },
            CommandType::L_COMMAND => {
                self.current.replace("(", "").replace(")", "")
            }
            _ => panic!(),
        }
    }

    pub fn dest(&self) -> String {
        if self.current.contains("=") {
            self.current.split("=").next().unwrap().to_string()
        } else {
            String::from("null")
        }
    }

    pub fn comp(&self) -> String {
        let mut command = self.current.clone();
        if let Some(pos) = command.find("=") {
            command = command.split_off(pos + 1);
        }
        if let Some(pos) = command.find(";") {
            command.truncate(pos);
        }
        command
    }

    pub fn jump(&self) -> String {
        if let Some(pos) = self.current.find(";") {
            self.current.clone().split_off(pos + 1)
        } else {
            String::from("null")
        }
    }
}
