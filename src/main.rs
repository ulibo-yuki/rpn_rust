use regex::Regex;

fn get_input() -> String { //String型で入力を返す
    let mut word = String::new();
    std::io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    word.trim().to_string()
}

fn check_halfspace(checked_string: &String) -> bool {
    let re = Regex::new(r"(\d\s\W)+$").unwrap();
    if re.is_match(&checked_string) {
        true
    } else {
        println!("演算子の間にスペースが含まれていない可能性があります。もう一度入力してください。\n");
        false
    }
}

fn check_length(checked_string: &String) -> bool{
    if checked_string.len() == 0 {
        println!("式が入力されていない可能性があります。もう一度入力してください。\n");
        false
    } else {
        true
    }
}

fn check_unavailable_character(checked_string: &String) -> bool {
    let re = Regex::new("[^+\\-*/%1234567890 ]").unwrap();
    if re.is_match(&checked_string) {
        println!("計算不可能な文字が含まれています。もう1度入力してください\n");
        false
    } else {
        true
    }
}

fn check_syntax(checked_string: &String) -> bool { //入力に演算不可能な文字があった場合false
    if check_unavailable_character(checked_string) == false {
        false
    } else if check_length(checked_string) == false {
        false
    } else if check_halfspace(checked_string) == false{
        false
    } else {
        true
    }
}

fn delimit(input: &String) -> Vec<&str>{ //文字列を空白で区切りベクタにして返す
    input.split_whitespace().collect()
}

fn is_numeric(input: &str) -> bool { //入力が数値ならtrue, 演算子ならfalse
    match input.parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn calculation(operand_1: f64, operand_2: f64, operator: &str) -> f64 { //演算
    match operator {
        "+" => operand_1 + operand_2,
        "-" => operand_1 - operand_2,
        "*" => operand_1 * operand_2,
        "/" => operand_1 / operand_2,
        "%" => operand_1 % operand_2,
        "**" => power(operand_1, operand_2),
        _ => 0.0,
    }
}

fn power(operand_1: f64, operand_2: f64) -> f64 { //指数演算
    let mut power_result = operand_1;
    for _ in 0..operand_2 as i64 - 1 {
        power_result *= operand_1
    }
    power_result
}

fn stack_manage(delimited_input: Vec<&str>) -> f64{
    let mut stack = Vec::<f64>::new();
    let mut result = if is_numeric(delimited_input[0]) == true {delimited_input[0].parse::<f64>().unwrap_or(0.0)} else {0.0};
    for i in delimited_input {
        if is_numeric(i) == true { //オペランドの場合
            stack.push(i.parse::<f64>().unwrap_or(0.0));
        } else { //演算子の場合
            if stack.len() < 2 {
                println!("被演算子(数)が足りない可能性があります。もう一度入力してください。\n");
                continue;
            }
            result =  calculation(stack[stack.len() - 2], stack[stack.len() - 1], i);
            for _ in 0..2 {
                stack.remove(stack.len() - 1);
            }
            stack.push(result); //結果の挿入
        }
    }
    result
}

fn main() {
    println!("rpn_rust: ver.1.3\n");
    loop {
        println!("式を入力してください。\n例: 1 + 2 → 1 2 +\n値や演算子同士は半角スペースで区切ってください。");
        let input_formula = get_input();
        if check_syntax(&input_formula) == false {continue;}
        let delimited_input_fomula = delimit(&input_formula);
        let result = stack_manage(delimited_input_fomula);
        println!("{}\nもう一度計算しますか?(y/n)", result);
        if get_input() == "n".to_string() {
            break;
        }
    }
}