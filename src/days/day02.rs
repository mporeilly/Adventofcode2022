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
            + match filedata[i] {
                // Unique style but I enjoy the simplicity
                b'A' => 1,
                b'B' => 2,
                b'C' => 3,
                _ => panic!(
                    "Unknown value: only able to recognize A, B or C within the first column"
                ),
            };
        // A for Rock, B for Paper, and C for Scissors - first column
        // X for Rock, Y for Paper, and Z for Scissors - second column

        // 0 if you lost, 3 if the round was a draw, and 6 if you won

        if filedata[i] == filedata[i + 1] {
            // tie
            points_collected = points_collected + 3;
        }
        // rock
        if filedata[i] == b'A' && filedata[i + 1] == b'Y' {
            // rock vs paper = loss
            points_collected = points_collected + 0;
        }
        if filedata[i] == b'A' && filedata[i + 1] == b'Z' {
            // rock vs sicssors = win
            points_collected = points_collected + 6;
        }
        // paper
        if filedata[i] == b'B' && filedata[i + 1] == b'Z' {
            // paper vs scissors = loss
            points_collected = points_collected + 0;
        }
        if filedata[i] == b'B' && filedata[i + 1] == b'X' {
            // paper vs rock = win
            points_collected = points_collected + 6;
        }
        // scissors
        if filedata[i] == b'C' && filedata[i + 1] == b'X' {
            // scissors vs rock = loss
            points_collected = points_collected + 0;
        }
        if filedata[i] == b'C' && filedata[i + 1] == b'Y' {
            // scissors vs paper = win
            points_collected = points_collected + 6;
        }

        i += 2;
        //print!("{:?}",i);
    }

    println!("Day 02 Part 01: {:?}", points_collected);
}
