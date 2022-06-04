use std::io::stdin;

pub fn color_to_code(color: &str) -> &str {
    let code: &str;
    match color {
        "green" => code = "\u{001b}[32m",
        _ => code = "\u{001b}[0m",
    }
    return code;
}

pub fn get_input(prompt: &str, color: &str) -> String {
    let color_code = color_to_code(color);
    // Read input string 
    let mut input = String::new();
    print!("{}", prompt);
    // Color input text
    println!("{} ", color_code);
    stdin().read_line(&mut input)
        .expect("Failed to read input :(");
    // Reset colors
    print!("\u{001b}[0m"); 
    // Only look at first line (remove "\n")
    match input.lines().next() {
        Some(value) => input = value.to_string(),
        None => println!("Could not read first line"),
    }
    return input;
}

pub fn get_option(prompt: &str, options: Vec<&str>, color: &str) -> String {
    loop {
        let input = get_input(prompt, color);
        // Input is valid option
        if options.iter().any(|&e| e == input) {
            return input;
        }
    }
}
