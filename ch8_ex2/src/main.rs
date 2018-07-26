fn pig_latinize(w : &str) -> String {
    let mut res = String::new();
    let mut first_char : Option<char> = None;
    let mut is_vowel = false;
    for c in w.chars() {
        if first_char.is_none() {
            first_char = Some(c);
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    is_vowel = true;
                    res.push(c);
                },
                _ => ()
            }
        } else {
            res.push(c);
        }
    }
    match first_char {
        Some(c) => {
            if is_vowel { res.push('h'); } else { res.push(c); }
            res = res + "ay";
        },
        _ => ()
    }

    return res;
}

fn main() {
    let words = vec!["first", "second", "apple", "asphalt", "egg"];
    for w in words {
        println!("{} -> {}", w, pig_latinize(w));
    }
}
