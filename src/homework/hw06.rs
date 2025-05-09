
fn draw_tree(triangle_count: u32) {
    let mut base_width = 1;
    let start = triangle_count as i32;

    for triangle in 0..triangle_count + 1 {
        for row in 0..=triangle {
            let stars = (2 * row + 1) as usize;
            let spaces = (start - row as i32) as usize;
            let line = " ".repeat(spaces) + &"*".repeat(stars);
            println!("{}", line);

            if row == triangle && triangle == triangle_count - 1 {
                base_width = stars;
            }
        }
    }
    let spaces = base_width / 2;
    println!("{}", " ".repeat(spaces));
}

fn main() {
    let triangle_count = 5;
    draw_tree(triangle_count);
}
