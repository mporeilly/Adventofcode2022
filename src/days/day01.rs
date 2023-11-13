use std;

// ACCII for the different symbols
const CARRAIGE_RETURN: u8 = 13;
static NEWLINE: u8 = 10;

// did think about it but went back to the "CULT OF DONE" mentality
// struct Elve {
//     calories: Vec<i32>,
// }

pub fn day01answers() {
    let filedata = std::fs::read("data/day01.txt").unwrap(); // started to fail had to remove "data/day01.txt" to "day01.txt" back to working
                                                        // need to have the carraige return and newline character in acii
    let mut staging_vector = Vec::new();
    // need a placeholder vector to hold the numbers
    let mut elve_calories: Vec<i32> = Vec::new();
    let mut elve_total: Vec<i32> = Vec::new();

    for i in 0..filedata.len() {
        //

        // file data is ACCII still need to take bytes and convert them to ints

        if filedata[i] == CARRAIGE_RETURN {
            // skip the carraiage return character
            continue;
        }

        if filedata[i] == NEWLINE {
            // newline signifies that it has moved on to the next item
            //println!("{:?}",staging_vector);
            // need to compress the vec to a single int
            //println!("{:?}",staging_vector.iter().collect::<String>() as i32);

            let calorie_placeholder = staging_vector.iter().collect::<String>(); // my first use of the turbo fish!

            if calorie_placeholder.len() > 0 {
                // skips over ones with no values
                let calorie_value = calorie_placeholder.parse::<i32>().unwrap(); // convert the string to i32
                                                                                 //println!("{:?}", calorie_value);
                elve_calories.push(calorie_value);
            }

            if staging_vector.len() == 0 {
                // length of zero signifies that there was two newlines back to back meaning there values are for a new elve
                //println!("new elve");
                // sum the elve_vector
                elve_total.push(elve_calories.iter().sum());
                elve_calories.clear();
            }
            // clear staging
            staging_vector.clear();
            // continue aka skip this "Newline" index
            continue;
        }

        // print!("{:?} ", filedata[i]);                           // no change
        // print!("{:?} ", i8::from_be_bytes([filedata[i]; 1]));   // no change
        // print!("{:?} ", filedata[i]as char);                    // works at converting the u8 accii to number
        staging_vector.push(filedata[i] as char); //.try_into().expect("miss match"));
                                                  //print!("{:?}",staging_vector);
    }
    // if not newline or carraige return then populate vector
    // if newline
    let max_calories = elve_total.iter().max().unwrap();
    println!(
        "Day 01 Part 01: Number of elves {:?}, elve {:?} has the most calories at {:?}",
        elve_total.len(),
        elve_total
            .iter()
            .position(|element| element == max_calories) // syntax is very clean, can use it for the next step in problem one
            .unwrap(),
        max_calories
    );
    // need total calories of the top 3 elves
    // sort and then grab top 3
    let mut sort = elve_total;
    sort.sort_by(|a, b| b.cmp(a)); // this puts the largest to smallest

    println!(
        "Day 01 Part 02: The total calories of the top three elves is: {:?}",
        &sort[0..3].iter().fold(0, |accumulate, x| accumulate + x)
    );
}
