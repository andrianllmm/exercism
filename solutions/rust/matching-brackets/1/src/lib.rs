pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for c in string.chars() {
        if "([{".contains(c) {
            stack.push(c);
        } else {
            let expected = match c {
                ')' => '(',
                ']' => '[',
                '}' => '{',
                _ => continue,
            };

            if stack.pop() != Some(expected) {
                return false;
            }
        }
    }

    stack.is_empty()
}
