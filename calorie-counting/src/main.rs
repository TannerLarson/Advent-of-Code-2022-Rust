// https://adventofcode.com/2022/day/1
fn main() {
    println!("{}", b())
}

fn a() -> u32 {
    let data = include_str!("../input.txt") // Read file into string
        .split("\n\n")// Make iterator for each elf (each it looks like "1000\n2000\n3000")
        .map( // Function that does something (`e.lines()`) to each iterator value
              |e| e.lines()  // Make iterator for each calorie number
                  .map(|c| c.parse::<u32>().unwrap()) // Convert string to u32
                  .sum::<u32>())  // Add up elf calorie total
        .max()
        .unwrap();
    /*
        Essentially what's happening here is a lot of messing with iterators. We start by making
        an elf iterator where each iterator value is a single elf. Then, we do two things with that
        iterator by using `.map()`. First, we make a second calorie iterator for each elf iterator,
        and turn each value in the calorie iterator into an unsigned integer. Using this new
        calorie iterator for each elf iterator, we can find the sum of the calories for each elf.
        At this point, each value in our elf iterator has a single unsigned int instead of a long
        string seperated by "\n". So, we can easily call the `max()` method on this iterator to get
        the result with the biggest value. Finally, we need to unwrap the result in order to extract
        the raw unsigned int, which we do on the last line.
    */
    return data
}

fn b() -> u32 {
    let mut elf_calories = include_str!("../input.txt") // This is mutable now because it needs to be sorted
        .split("\n\n")
        .map(|e| e.lines()
                  .map(|c| c.parse::<u32>().unwrap())
                  .sum::<u32>())
        .collect::<Vec<u32>>(); // Collect calorie counts into a vector
    elf_calories.sort();
    return elf_calories.iter() // Turn vector into iterator
        .rev() // Set iterator to go from right to left
        .take(3) // Create a sub-iterator that covers only the first three iterator values
        .sum(); // Get the sum of our new iterator
}

// fn test() {
//     let x = "Hello\nthere,\nmy\nname\nis\ntanner".lines().collect();
//     let vec: Vec<&str> = x;
//     println!("Hello");
// }

