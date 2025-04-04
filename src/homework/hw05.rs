fn gcd0(a: i32, b: i32) -> i32 {
    if b == 0 {a} else {gcd0(b, a % b)}
}


fn gcd(a: i32, b: i32) -> i32 {
    if a > b{gcd0(a,b)} else{gcd0(b, a)}
}

fn test() {
    let data =
        [
            ((24,  60), 12),
            ((15,   9),  3),
            ((15,   6),  3),
            ((140, 40), 20),
            ((24,  16),  8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37,  11),  1),
            ((120, 90), 30),
        ];


    for ((a, b), exp) in data.iter() {
        assert_eq!(*exp, gcd(*a, *b));
    }
}


fn main() {
    let  a:i32 = 1028;
    let  b:i32 = 2900;


    println!("{}", gcd(a, b));
    test();
}

