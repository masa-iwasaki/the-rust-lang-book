fn main() {
    // let word = String::from("apple");
    let word = String::from("first");

    println!("{}", format!("{}-{}", make_body(&word), make_suffix(&word)));
}

fn is_vowel(c: String) -> bool {
    if c == "a" || c == "e" || c == "i" || c == "o" || c == "u"  {
        true
    } else {
        false
    }
}

fn make_body(word: &String) -> String {
    let mut chars = word.chars();
    let head: String = match chars.next() {
        None => String::new(),
        Some(i) => i.to_string(),
    };

    if is_vowel(String::from(head.as_str())) {
        String::from(word)
    } else {
        let mut ret = String::from("");
        for c in word.to_string().chars() {
            ret.push(c)
        }
        String::from(ret)
    }
}

fn make_suffix(word: &String) -> String {
    let mut chars = word.chars();
    let head: String = match chars.next() {
        None => String::new(),
        Some(i) => i.to_string(),
    };

    if is_vowel(String::from(head.as_str())) {
        String::from("hay")
    } else {
        let mut ret = String::from(head.as_str());
        ret.push_str("ay");
        ret.to_string()
    }
}
