use rand::Rng;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(10..=50);
    let mut shipments = vec![avg; n];

    for _ in 0..(n * 3) {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        if i != j && shipments[i] > 0 {
            shipments[i] -= 1;
            shipments[j] += 1;
        }
    }

    shipments
}

fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return -1;
    }

    let avg = total / n;
    let mut moves = 0;

    for &weight in shipments {
        if weight > avg {
            moves += (weight - avg) as i32;
        }
    }

    moves
}

fn print_shipments_info(shipments: &[u32]) {
    println!("Indexes: ");
    for i in 0..shipments.len() {
        print!("{:>3}", i);
    }
    println!();

    println!("Shipments:");
    for s in shipments {
        print!("{:>3}", s);
    }
    println!();
}

#[test]
fn test() {
    let shipments = gen_shipments(10);

    print_shipments_info(&shipments);

    let result = count_permutation(&shipments);
    if result == -1 {
        println!("Equal distribution impossible. Output: -1");
    } else {
        println!("Min moves needed: {}", result);
    }
}