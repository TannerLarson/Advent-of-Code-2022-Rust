
fn main() {
    println!("{}", b());
}

// Get pairs where one pare completely encompasses the other
fn a() -> usize {
    include_str!("../input.txt").lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap(); // Split into two elf assignments
            let ((a, b), (c,d)) =
                (left.split_once('-').unwrap(),
                 right.split_once('-').unwrap()); // get lower and upper bounds for both elves
            ( // extract number from bounds
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
                )
        })
        // Use a couple and statements to check if the number ranges meet the criteria
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (c >= a && d <= b))
        .count() // Count the amount of times the criteria was met
}

// Get pairs where there is overlap between two paris
fn b() -> usize {
    include_str!("../input.txt").lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap(); // Split into two elf assignments
            let ((a, b), (c,d)) =
                (left.split_once('-').unwrap(),
                 right.split_once('-').unwrap()); // get lower and upper bounds for both elves
            ( // extract number from bounds
              a.parse::<u8>().unwrap(),
              b.parse::<u8>().unwrap(),
              c.parse::<u8>().unwrap(),
              d.parse::<u8>().unwrap(),
            )
        })
        // Use a couple and statements to check if the number ranges meet the criteria
        .filter(|(a, b, c, d)| (a <= c && c <= b) || (c <= a && a <= d))
        .count() // Count the amount of times the criteria was met
}