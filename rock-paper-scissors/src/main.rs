
fn main() {
    println!("{}",include_str!("../input.txt").split("\n")
        .map(|line| get_player_points(
            line.chars().nth(2).unwrap(), line.chars().nth(0).unwrap()))
        .sum::<u32>());
}

// // a
// fn get_player_points(player: char, oponnent: char) -> u32 {
//     let p = match player {
//         'X' => "rock",
//         'Y' => "paper",
//         'Z' => "scissors",
//         _ => panic!("Incorrect player value: {}", player)
//     };
//     let o = match oponnent {
//         'A' => "rock",
//         'B' => "paper",
//         'C' => "scissors",
//         _ => panic!("Incorrect oponnent value: {}", oponnent)
//     };
//     let points = match p {
//         "rock" => 1,
//         "paper" => 2,
//         "scissors" => 3,
//         _ => panic!("Incorrect p value: {}", p)
//     };
//     if p == o { return points + 3 }
//     if (p == "rock" && o == "scissors") ||
//         (p == "paper" && o == "rock") ||
//         (p == "scissors" && o == "paper") {
//         return points + 6
//     }
//     return points
// }

// b
fn get_player_points(result: char, oponnent: char) -> u32 {
    let player = if result == 'Y' { oponnent } else {
        match oponnent {
            'A' => if result == 'Z' { 'B' } else { 'C' },
            'B' => if result == 'Z' { 'C' } else { 'A' },
            'C' => if result == 'Z' { 'A' } else { 'B' },
            _ => panic!("Value Error: {}", oponnent)
        }
    };
    let choice_points = match player {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => panic!("Value Error")
    };
    return choice_points + match result {
        'X' => 0,
        'Y' => 3,
        'Z' => 6x,
        _ => panic!("Value Error")
    };
}
