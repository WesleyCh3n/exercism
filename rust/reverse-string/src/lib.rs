pub fn reverse(input: &str) -> String {
    // input.chars().fold(String::new(), move |mut word, c| { word.push(c) })
    let mut s = String::new();
    let mut inp = input.to_string();
    while !inp.is_empty() {
        s.push(inp.pop().unwrap());
    }
    s
}
