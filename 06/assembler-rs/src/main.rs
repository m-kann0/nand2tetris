mod modules;
use modules::Parser;
use modules::CommandType;
use modules::code;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let mut parser = Parser::new(&filename);
    while parser.has_more_content() {
        parser.advance();
//        println!("{:?}", parser);
        match parser.command_type() {
            CommandType::A_COMMAND => {
                let symbol = parser.symbol();
                let binary = format!("{:016b}", symbol.parse::<usize>().unwrap());
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
            _ => panic!(),
        }
    }
}
