extern crate csv;
extern crate rand;
use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let names: Vec<Vec<String>> = prompt_for_and_read_csv();
    let names = sort_families(names);

    println!("Enter file path for a text list of previous years' giving");
    let previous_years_file_path = get_file_path();
    let previous_years_giving: Vec<String> = read_by_line(&previous_years_file_path).unwrap();

    // loop until we get a good solution
    loop {
        match find_gift_givers(&names, &previous_years_giving) {
            Some(_vec) => break,
            None => {
                println!("\nGot a bad solution\nGoing to try again\n");
                continue;
            }
        };
    }
}

fn find_gift_givers(
    names: &Vec<Vec<String>>,
    previous_years_giving: &Vec<String>,
) -> Option<Vec<String>> {
    let mut receiving_vec: Vec<String> = [].to_vec();
    for (family_number, family) in names.iter().enumerate() {
        // family_number is a counter here... it's like an each_with_index
        for giver in family {
            match find_receiver_for(
                giver,
                family_number,
                &names,
                &receiving_vec,
                previous_years_giving,
            ) {
                Some(name) => receiving_vec.push(name),
                None => return None, // println!("Couldn't find solution. Please run program again."),
            }
        }
    }
    Some(receiving_vec)
}

fn find_receiver_for(
    giver_name: &str,
    giver_family_number: usize,
    names: &Vec<Vec<String>>,
    receiving_vec: &Vec<String>,
    previous_years_giving: &Vec<String>,
) -> Option<String> {
    let mut rng = thread_rng();
    let mut potential_receiver_family_number;
    let mut potential_receiver_member_number;
    let mut potential_receiver_name;
    let mut loop_counter = 0;
    loop {
        loop_counter += 1;
        if loop_counter > 1000 {
            // We painted ourselves into a corner!
            return None;
        }

        potential_receiver_family_number = rng.gen_range(0, names.len());
        potential_receiver_member_number =
            rng.gen_range(0, names[potential_receiver_family_number].len());
        potential_receiver_name =
            &names[potential_receiver_family_number][potential_receiver_member_number];

        // what makes a bad receiver?
        //   - potential receiver is already receiving
        //   - potential receiver IS this giver
        //   - potential receiver is in this giver's family
        if receiving_vec.contains(&potential_receiver_name.to_string())
            || potential_receiver_name == giver_name
            || giver_family_number == potential_receiver_family_number
        {
            // go to the next iteration of the loop
            continue;
        } else if previous_years_giving.contains(&format!(
            "{} is giving to {}",
            giver_name, potential_receiver_name
        )) {
            // This giver gave to this person in previous years. That's no fun!
            continue;
        } else {
            // if I'm here, I know I have got a good one. let's break out of the loop and push
            break;
        }
    }

    println!("{} is giving to {}", giver_name, potential_receiver_name);

    Some(potential_receiver_name.to_string())
}

fn get_file_path() -> String {
    let file_path = gets().unwrap();
    let file_path = file_path.trim_matches(|c| c == '\'' || c == ' ');
    file_path.to_string()
}

fn prompt_for_and_read_csv() -> Vec<Vec<String>> {
    let mut names: Vec<Vec<String>> = [].to_vec();

    // let file_path = "test-names.csv";
    println!("Enter the file path of the csv file");
    let file_path = gets().unwrap();
    let file_path = file_path.trim_matches(|c| c == '\'' || c == ' ');

    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        let record = result.expect("a CSV record");
        let mut family_vec_strings: Vec<String> = [].to_vec();
        // let family_vec: Vec<&str> = record.iter().collect();
        for name in record.iter() {
            if name.len() > 1 {
                family_vec_strings.push(name.to_string());
            }
        }

        names.push(family_vec_strings);
    }
    names
}

fn sort_families(mut names: Vec<Vec<String>>) -> Vec<Vec<String>> {
    names.sort_by(|family1, family2| family1.len().cmp(&family2.len()));
    names.reverse();
    names
}

// helper functions (also in sts10/eyeoh)
fn gets() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches("\n").to_string()),
        Err(error) => Err(error),
    }
}

fn read_by_line<T: FromStr>(file_path: &str) -> io::Result<Vec<T>> {
    let mut vec = Vec::new();
    let f = match File::open(file_path.trim_matches(|c| c == '\'' || c == ' ')) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line?.parse() {
            Ok(l) => vec.push(l),
            Err(_e) => {
                eprintln!("Error");
                continue;
            }
        }
    }
    Ok(vec)
}
