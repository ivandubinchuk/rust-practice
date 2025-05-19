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
#[test]
fn test() {
    let data =
        [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];


    data
        .iter()
        .for_each(|(a, b)| {
            assert_eq!(
                invert_the_case(a.to_string()),
                b.to_string()
            );
            assert_eq!(
                invert_the_case(b.to_string()),
                a.to_string()
            );
        });
}
