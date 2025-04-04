#[test]
fn main() {
    const W: u32 = 22;
    const H: u32 = 17;

    for y in 0..H {
        for x in 0..W {
            let is_hor = y == 0 || y == H - 1;
            let is_ver = x == 0 || x == W - 1;

            let is_diag1 = x as f32 == (y as f32 * (W - 1) as f32 / (H - 1) as f32).round();
            let is_diag2 = x as f32 == ((H - 1 - y) as f32 * (W - 1) as f32 / (H - 1) as f32).round();

            let sym = if is_hor || is_ver || is_diag1 || is_diag2 { '*' } else { ' ' };
            print!("{}", sym);
        }
        println!();
    }
}


