use std::collections::HashMap;
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    let map: HashMap<char, char> = vec![(')', '('), (']', '['), ('}', '{')]
        .into_iter()
        .collect();

    for c in s.chars() {
        if let Some(&opening) = map.get(&c) {
            if stack.pop() != Some(opening) {
                return false;
            }
        } else {
            stack.push(c);
        }
    }

    stack.is_empty()
}

