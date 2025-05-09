fn invert_the_case(s: String) -> String {
    let mut changes = String::new();
    for chr in s.chars() {
        if chr.is_lowercase() {
            changes.push(chr.to_uppercase().next().unwrap());
        } else if chr.is_uppercase() {
            changes.push(chr.to_lowercase().next().unwrap());
        } else {
            changes.push(chr);
        }
    }
    changes
}

fn test() {
    let data = [
        ("Hello", "hELLO"),
        ("Привет", "пРИВЕТ"),
    ];

    for (a, _) in &data {
        println!("{}: {}", a, invert_the_case(a.to_string()));
    }

    for (_, b) in &data {
        println!("{}: {}", b, invert_the_case(b.to_string()));
    }
}

fn main() {
    test();
}
