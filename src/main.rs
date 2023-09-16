use std;

// ACCII for the different symbols
const CARRAIGE_RETURN: u8 = 13;
static NEWLINE: u8 = 10;

struct Elve {
    
}

fn main() {
    let filedata = std::fs::read("text.txt").unwrap();
    // need to have the carraige return and newline character in acii
    let mut staging_vector = Vec::new();
    // need a placeholder vector to hold the numbers
    //let mut result_vector: Vec<u8>= Vec::new();

    for i in 0..50 {
        //filedata.len() { //

        // file data is ACCII still need to take bytes and convert them to ints   

        if filedata[i] == CARRAIGE_RETURN{
            // skip the carraiage return character
            continue;
        }
    
        if filedata[i] == NEWLINE{
            // newline signifies that it has moved on to the next item
            //println!("{:?}",staging_vector);
            // need to compress the vec to a single int
            //println!("{:?}",staging_vector.iter().collect::<String>() as i32);

            let mut calorie_placeholder = staging_vector.iter().collect::<String>();    // my first use of the turbo fish!
            let mut calorie_value = calorie_placeholder.parse::<i32>().unwrap();           // convert the string to i32 
            println!("{:?}", calorie_value);



            if staging_vector.len() == 0{
                // length of zero signifies that there was two newlines back to back meaning there values are for a new elve
                println!("new elve");
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
}
