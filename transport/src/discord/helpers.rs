pub fn split_with_quotes(input: String) -> (String, Vec<String>) {
    let mut result: Vec<String> = Vec::new();
    let mut current_word = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                if !in_quotes && !current_word.is_empty() {
                    result.push(current_word.clone());
                    current_word.clear();
                }
            }
            ' ' => {
                if !in_quotes && !current_word.is_empty() {
                    result.push(current_word.clone());
                    current_word.clear();
                }
            }
            _ => {
                current_word.push(c);
            }
        }
    }

    if !current_word.is_empty() {
        result.push(current_word);
    }

    if result.is_empty() {
        (String::new(), Vec::new())
    } else {
        let first_part = result.remove(0);
        (first_part, result)
    }
}
