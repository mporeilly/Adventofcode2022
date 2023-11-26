// adding this file causes the reading in of the "day01.txt" to fail???
// able to get it going by removing "data/day01.txt" to "day01.txt" this then failed later on and had to be switched back
static NEWLINE: u8 = 10;
static SPACE: u8 = 32;
pub fn day02answers() {
    let mut filedata = std::fs::read("data/day02.txt").unwrap();

    filedata.retain(|&x| (x != NEWLINE)); // remove unneeded symbols
    filedata.retain(|&x| (x != SPACE)); // remove unneeded symbols

    if filedata.len() % 2 != 0 {
        panic!("Number of entries is not even")
    }

    // splitting up the columns
    let mut i = 0;
    let mut first_column = vec![];
    let mut second_column = vec![];
    let mut points_collected = 0;

    while i < (filedata.len()) {
        //println!("first column: {:?}, Second column: {:?}", filedata[i] as char, filedata[i+1] as char);
        first_column.push(filedata[i] as char);
        second_column.push(filedata[i + 1] as char);
        // 1 for Rock, 2 for Paper, and 3 for Scissors
        points_collected = points_collected
            + match filedata[i + 1] {
                // Unique style but I enjoy the simplicity
                b'X' => 1,
                b'Y' => 2,
                b'Z' => 3,
                _ => panic!(
                    "Unknown value: only able to recognize A, B or C within the first column"
                ),
            };
        // A for Rock, B for Paper, and C for Scissors - first column
        // X for Rock, Y for Paper, and Z for Scissors - second column (what I played)

        // 0 if you lost, 3 if the round was a draw, and 6 if you won

        // use ascii to allow the values to be compared  based on an offset A = 65 (rock) X = 88 (rock)
        if filedata[i] + (b'X' - b'A') == filedata[i + 1] {
            // tie
            points_collected = points_collected + 3;
        }
        // rock
        if filedata[i + 1] == b'Y' && filedata[i] == b'A' {
            // p vs r = win
            points_collected = points_collected + 6;
        }
        if filedata[i + 1] == b'Z' && filedata[i] == b'A' {
            // s vs r  = loss
            points_collected = points_collected + 0;
        }
        // paper
        if filedata[i + 1] == b'Z' && filedata[i] == b'B' {
            // s vs p = win
            points_collected = points_collected + 6;
        }
        if filedata[i + 1] == b'X' && filedata[i] == b'B' {
            // r vs p = loss
            points_collected = points_collected + 0;
        }
        // scissors
        if filedata[i + 1] == b'X' && filedata[i] == b'C' {
            // r vs s = win
            points_collected = points_collected + 6;
        }
        if filedata[i + 1] == b'Y' && filedata[i] == b'C' {
            // p vs s = loss
            points_collected = points_collected + 0;
        }

        i += 2;
        //print!("{:?}",i);
    }

    //X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win

    println!("Day 02 Part 01: If everything goes to plan total points: {:?}", points_collected);
}
