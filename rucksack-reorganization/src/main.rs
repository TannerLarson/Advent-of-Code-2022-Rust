fn main() {
    println!("{}", b());
}

fn get_priority(item: char) -> u32 {
    let ascii_value = item as u32;
    ascii_value - if item.is_uppercase() { 38 } else { 96 }
}

fn a() -> u32 {
    include_str!("../input.txt").split("\n")
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2)) // Split string in half
        .map(|(first, second)| second.chars() // Iterate through second compartment
            .filter(|c| first.contains(*c)) // Only yield a value in the iterator if the value passes the check. `c` is initially a reference to a character (&char), and because the `collect` method only allows a normal char input (char), we must dereference it with a `*`
            .map(|shared_item| get_priority(shared_item))
            .next().unwrap()) // At this point, we have an iterator but we need an actual value. Because we know that the filter will only yield a single value (see the original question), we only need to get a single value by calling `next()`
        .sum() // Get the priority
}

fn b() -> u32 {
    include_bytes!("../input.txt") // Create a byte array from the input, which is pretty much an array where each element is the ascii version of the character
        .split(  // Create an iterator that iterates over every line
            |line| *line // We must dereference b `line` here because the `==` operator can't compare a reference (&u8) to a value (l8)
                == b'\n') // The `b` prefix indicates that the string should be treated as a byte sequence
        .collect::<Vec<_>>().chunks(3) // Collect iterator into a vector in order to iterate over chunks of three
        .map(|elf_group|
            elf_group[0].iter()
                .filter(|c| elf_group[1].contains(*c) && elf_group[2].contains(*c))
                .map(|badge| get_priority(*badge as char))
                .next().unwrap())
        .sum()
}