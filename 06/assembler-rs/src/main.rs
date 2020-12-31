mod modules;
use modules::CommandType;
use modules::code;
use modules::Parser;
use modules::SymbolTable;
use std::env;

fn main() {
    let mut table = SymbolTable::new();
    table.add_entry("SP".to_string(), 0);
    table.add_entry("LCL".to_string(), 1);
    table.add_entry("ARG".to_string(), 2);
    table.add_entry("THIS".to_string(), 3);
    table.add_entry("THAT".to_string(), 4);
    for i in 0..16 {
        let symbol = format!("R{}", i);
        table.add_entry(symbol, i);
    }
    table.add_entry("SCREEN".to_string(), 16384);
    table.add_entry("KBD".to_string(), 24576);

    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    let mut parser = Parser::new(&filename);
    let mut address = 0;
    while parser.has_more_content() {
        parser.advance();
        match parser.command_type() {
            CommandType::A_COMMAND | CommandType::C_COMMAND => {
                address += 1;
            },
            CommandType::L_COMMAND => {
                let symbol = parser.symbol();
                table.add_entry(symbol, address);
            },
        }
    }

    let mut parser = Parser::new(&filename);
    let mut address_seq = 16;
    while parser.has_more_content() {
        parser.advance();
        match parser.command_type() {
            CommandType::A_COMMAND => {
                let symbol = parser.symbol();
                let address = if symbol.chars().next().unwrap().is_digit(10) {
                    symbol.parse::<usize>().unwrap()
                } else if table.contains(symbol.clone()) {
                    table.get_address(symbol.clone())
                } else {
                    let a = address_seq;
                    address_seq += 1;
                    table.add_entry(symbol, a);
                    a
                };
                let binary = format!("{:016b}", address);
                println!("{}", binary);
            },
            CommandType::C_COMMAND => {
                let dest = parser.dest();
                let comp = parser.comp();
                let jump = parser.jump();
                let mut binary = String::from("111");
                binary.push_str(code::comp(&comp));
                binary.push_str(code::dest(&dest));
                binary.push_str(code::jump(&jump));
                println!("{}", binary);
            },
            _ => {
                // ignore
            },
        }
    }
}
