use itertools::Itertools;
fn solve() {
    let letters = ['m', 'u', 'x', 'a', 's', 'l', 'o', 'n'];

    let mut count = 0;

    for perm in (0..10).permutations(8) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        if m == 0 {
            continue;
        }
        let muxa = m * 1000 + u * 100 + x * 10 + a;

        if a == 0 {
            continue;
        }

        if s == 0 {
            continue;
        }
        let slon = s * 1000 + l * 100 + o * 10 + n;

        if muxa * a == slon {
            println!(
                "  {}{}{}{}\n×    {}\n-------\n  {}{}{}{}",
                m, u, x, a, a, s, l, o, n
            );
            println!();
            count += 1;
        }
    }

    println!("Total solutions: {}", count);
}

#[test]
fn test(){
    solve();
}