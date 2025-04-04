fn draw_romb(size: u32) {
    let half = size / 2;

    for y in 0..size {
        let abs_y = if y < half { y } else { size - y - 1 };
        let spaces = half - abs_y;
        let stars = size - spaces * 2;

        for _ in 0..spaces {
            print!(" ");
        }
        for _ in 0..stars {
            print!("*");
        }
        println!();
    }
}


#[test]
fn main() {
    const S: u32 = 11;
    draw_romb(S);
}