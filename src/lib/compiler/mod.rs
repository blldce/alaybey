pub mod language {
    use std::collections::HashMap;

    const MAX_NUM: i32 = 536870911; // 11111111111111111111111111111
    const VAR_SGN: i32 = 1073741833;
    const HALT_NSTRUCTON: i32 = 0x40000000;
    const VAR_USAGE_START_PONT: u32 = 0x60000000_u32;
    const VAR_DEF_START_PONT: u32 = 0xA0000000_u32;

    pub struct Token {
        pub syntax_token: String,
    }

    pub fn check_lexer(lexers: Vec<&str>) {
        if lexers.first().eq(&Option::Some(&"{")) && lexers.last().eq(&Option::Some(&"}")) {
            let mut var_name_list = Vec::<String>::new();

            &lexers[1..lexers.len() - 1]
                .iter()
                .filter(|val| {
                    let semi_colon = &val
                        .chars()
                        .collect::<Vec<char>>()
                        .last()
                        .unwrap()
                        .to_owned();

                    if semi_colon.eq(&';') {
                        true
                    } else {
                        panic!("syntax error.. Semicolon missing!");
                    }
                })
                .map(|line| {
                    let vec_without_semi = line.split(';').collect::<Vec<_>>();
                    let mut is_done_line = false;
                    let line_chars = vec_without_semi[..vec_without_semi.len() - 1]
                        .iter()
                        .collect::<Vec<_>>()
                        .first()
                        .unwrap()
                        .trim()
                        .chars()
                        .collect::<Vec<char>>();

                    line_chars
                        .iter()
                        .filter(|ch| {
                            if !is_done_line {
                                is_done_line = true;
                                if ch.eq(&&'$') {
                                    if line_chars.contains(&':') {
                                        let mut eq_index = 0;
                                        for (i, ch) in line_chars.iter().enumerate() {
                                            if ch.eq(&':') {
                                                eq_index = i;
                                                break;
                                            }
                                        }
                                        let var_name_chars = &line_chars[0..eq_index];
                                        if var_name_chars[0].eq(&'$') {
                                            let mut var_str =
                                                var_name_chars[1..].iter().collect::<String>();
                                            var_str.retain(|c| !c.is_whitespace());
                                            if !var_name_list.contains(&var_str) {
                                                var_name_list.push(var_str);
                                            } else {
                                                panic!("this variable defined before: {}", var_str);
                                            }
                                        } else {
                                            panic!("variable must start with $ sign!")
                                        }
                                        let mut operation_str =
                                            line_chars[eq_index + 1..].iter().collect::<String>();
                                        operation_str.retain(|c| !c.is_whitespace());

                                        if operation_str.len() > 2 {
                                            let var_name_chars = &line_chars[eq_index + 1..];
                                            let mut lines = Vec::<String>::new();
                                            let mut is_next_str = false;
                                            for character in var_name_chars {
                                                if !character.eq(&' ') {
                                                    if is_next_str {
                                                        let last = lines.last_mut().unwrap();
                                                        last.push(*character);
                                                        continue;
                                                    }
                                                    lines.push(format!("{}", character));
                                                    is_next_str = true;
                                                } else {
                                                    is_next_str = false;
                                                }
                                            }
                                            check_line_num_val(lines, &var_name_list);
                                        } else {
                                            panic!("invalid syntax.. Pattern error!")
                                        }
                                        true
                                    } else {
                                        panic!("variable must initialize with ':' ")
                                    }
                                } else {
                                    // no $ sign
                                    let mut operation_str =
                                        line_chars[..].iter().collect::<String>();
                                    operation_str.retain(|c| !c.is_whitespace());
                                    if operation_str.len() > 2 {
                                        let var_name_chars = &line_chars[..];
                                        let mut new_vec = Vec::<String>::new();
                                        let mut is_next_str = false;
                                        for character in var_name_chars {
                                            if !character.eq(&' ') {
                                                if is_next_str {
                                                    let last = new_vec.last_mut().unwrap();
                                                    last.push(*character);
                                                    continue;
                                                }
                                                new_vec.push(format!("{}", character));
                                                is_next_str = true;
                                            } else {
                                                is_next_str = false;
                                            }
                                        }
                                        check_line_num_val(new_vec, &var_name_list);
                                    } else {
                                        panic!("invalid syntax.. Pattern error!")
                                    }
                                    true
                                }
                            } else {
                                true
                            }
                        })
                        .count();
                })
                .count();
        } else {
            panic!("syntax error.. Brackets missing!")
        }
    }

    fn check_line_num_val(new_vec: Vec<String>, var_name_list: &[String]) {
        let mut is_op = true;
        let mut first_two_cnt = 0;
        new_vec
            .iter()
            .filter(|ch| {
                if first_two_cnt < 2 {
                    let is_digit = ch.parse::<i32>().is_ok();
                    if is_digit {
                        is_op = true;
                    } else if var_name_list.contains(&ch) {
                        is_op = true;
                    } else {
                        panic!("variable not defined before: {}", ch)
                    }
                    first_two_cnt += 1;
                    true
                } else {
                    if is_op {
                        if check_op_code(ch.as_str()) {
                            is_op = false;
                            true
                        } else {
                            panic!("invalid syntax.. Has to be operator!")
                        }
                    } else {
                        let is_digit = ch.parse::<i32>().is_ok();
                        if is_digit {
                            is_op = true;
                            true
                        } else if var_name_list.contains(&ch) {
                            is_op = true;
                            true
                        } else {
                            panic!("variable not defined before!")
                        }
                    }
                }
            })
            .count();
    }

    fn check_op_code(lexer: &str) -> bool {
        match lexer {
            "+" | "-" | "*" | "/" | "%" => true,
            _ => {
                panic!("invalid syntax.. Unknown/No operator type!");
            }
        }
    }

    pub fn lexer(language_text: &str) -> Vec<Token> {
        check_lexer(
            language_text
                .split('\n')
                .collect::<Vec<&str>>()
                .iter()
                .map(|a| a.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>(),
        );

        let mut token_list: Vec<Token> = Vec::new();
        let mut char_list: Vec<char> = Vec::new();
        for character in language_text.chars() {
            match character {
                '\n' | '\t' | ':' => {
                    continue;
                }
                '{' => {
                    token_list.push(Token {
                        syntax_token: "{".to_string(),
                    });
                }
                ' ' => {
                    if !char_list.is_empty() {
                        token_list.push(Token {
                            syntax_token: char_list.iter().collect::<String>(),
                        });
                    }
                    char_list.clear();
                }
                ';' => {
                    if !char_list.is_empty() {
                        token_list.push(Token {
                            syntax_token: char_list.iter().collect::<String>().trim().to_string(),
                        });
                    }
                    token_list.push(Token {
                        syntax_token: ";".to_string(),
                    });
                    char_list.clear();
                }

                '}' => {
                    token_list.push(Token {
                        syntax_token: "}".to_string(),
                    });
                    break;
                }
                _ => {
                    char_list.push(character);
                }
            }
        }
        token_list
    }
    fn token_to_number(s: &str) -> i32 {
        s.parse().unwrap()
    }

    fn token_to_instruction(s: &str) -> u32 {
        match s {
            "+" => 0x40000001, // 1000000000000000000000000000001 (len 31)
            "-" => 0x40000002,
            "*" => 0x40000003,
            "/" => 0x40000004,
            "%" => 0x40000005,
            ";" => 0x40000006,
            "{" => 0x40000007,
            "}" => 0x40000008,
            "$" => 0x40000009,
            _ => {
                panic!("syntax error.. invalid instruction type!")
            }
        }
    }

    pub fn parser(token_list: Vec<Token>) -> Vec<i32> {
        let mut instruction_list: Vec<i32> = Vec::new();
        let mut var_map_definition = HashMap::<String, u32>::new();
        let mut var_definition = VAR_DEF_START_PONT;
        for token in token_list {
            let is_num = token.syntax_token.trim().parse::<i32>().is_ok();
            if is_num {
                let program_data = token_to_number(&token.syntax_token.trim().to_string());
                if program_data < MAX_NUM {
                    instruction_list.push(program_data);
                } else {
                    panic!(
                        "Out of range nuber! Maximum number should be less than {}",
                        MAX_NUM
                    );
                }
            } else {
                if let Some(last_data) = instruction_list.last() {
                    if *last_data == VAR_SGN {
                        if var_map_definition.contains_key(&token.syntax_token) {
                            panic!("variable error.. dublicate variable definition!")
                        } else {
                            var_definition += 1;
                            var_map_definition.insert(token.syntax_token, var_definition);
                            instruction_list.push(var_definition as i32);
                        }
                    } else {
                        if var_map_definition.contains_key(&token.syntax_token) {
                            let var_raw_value =
                                *var_map_definition.get(&token.syntax_token).unwrap();
                            let var_value = get_variable_value(var_raw_value as i32) as u32;
                            let var_pointer = VAR_USAGE_START_PONT + var_value;
                            instruction_list.push(var_pointer as i32);
                        } else {
                            instruction_list.push(
                                (token_to_instruction(&token.syntax_token.trim().to_string()))
                                    as i32,
                            )
                        }
                    }
                } else {
                    instruction_list
                        .push((token_to_instruction(&token.syntax_token.trim().to_string())) as i32)
                }
            }
        }
        instruction_list.push(HALT_NSTRUCTON); // HALT instruction
        instruction_list
    }

    fn get_variable_value(instruction: i32) -> i32 {
        instruction & 0xfffffff
    }
}
