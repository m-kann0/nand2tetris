pub fn dest(mnemonic: &str) -> &str {
    match mnemonic {
        "null" => "000",
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "AMD" => "111",
        _ => {
            eprintln!("unexpected parameter: {}", mnemonic);
            panic!();
        },
    }
}

pub fn comp(mnemonic: &str) -> &str {
    match mnemonic {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        _ => {
            eprintln!("unexpected parameter: {}", mnemonic);
            panic!();
        },
    }
}

pub fn jump(mnemonic: &str) -> &str {
    match mnemonic {
        "null" => "000",
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => {
            eprintln!("unexpected parameter: {}", mnemonic);
            panic!();
        },
    }
}

mod test {
    #[allow(unused_imports)]
    use crate::code;

    #[test]
    fn test_dest() {
        assert_eq!(code::dest("null"), "000");
        assert_eq!(code::dest("M"), "001");
        assert_eq!(code::dest("D"), "010");
        assert_eq!(code::dest("MD"), "011");
        assert_eq!(code::dest("A"), "100");
        assert_eq!(code::dest("AM"), "101");
        assert_eq!(code::dest("AD"), "110");
        assert_eq!(code::dest("AMD"), "111");
    }

    #[test]
    fn test_comp() {
        assert_eq!(code::comp("0"),   "0101010");
        assert_eq!(code::comp("1"),   "0111111");
        assert_eq!(code::comp("-1"),  "0111010");
        assert_eq!(code::comp("D"),   "0001100");
        assert_eq!(code::comp("A"),   "0110000");
        assert_eq!(code::comp("!D"),  "0001101");
        assert_eq!(code::comp("!A"),  "0110001");
        assert_eq!(code::comp("-D"),  "0001111");
        assert_eq!(code::comp("-A"),  "0110011");
        assert_eq!(code::comp("D+1"), "0011111");
        assert_eq!(code::comp("A+1"), "0110111");
        assert_eq!(code::comp("D-1"), "0001110");
        assert_eq!(code::comp("A-1"), "0110010");
        assert_eq!(code::comp("D+A"), "0000010");
        assert_eq!(code::comp("D-A"), "0010011");
        assert_eq!(code::comp("A-D"), "0000111");
        assert_eq!(code::comp("D&A"), "0000000");
        assert_eq!(code::comp("D|A"), "0010101");
        assert_eq!(code::comp("M"),   "1110000");
        assert_eq!(code::comp("!M"),  "1110001");
        assert_eq!(code::comp("-M"),  "1110011");
        assert_eq!(code::comp("M+1"), "1110111");
        assert_eq!(code::comp("M-1"), "1110010");
        assert_eq!(code::comp("D+M"), "1000010");
        assert_eq!(code::comp("D-M"), "1010011");
        assert_eq!(code::comp("M-D"), "1000111");
        assert_eq!(code::comp("D&M"), "1000000");
        assert_eq!(code::comp("D|M"), "1010101");
    }

    #[test]
    fn test_jump() {
        assert_eq!(code::jump("null"), "000");
        assert_eq!(code::jump("JGT"), "001");
        assert_eq!(code::jump("JEQ"), "010");
        assert_eq!(code::jump("JGE"), "011");
        assert_eq!(code::jump("JLT"), "100");
        assert_eq!(code::jump("JNE"), "101");
        assert_eq!(code::jump("JLE"), "110");
        assert_eq!(code::jump("JMP"), "111");
    }
}
