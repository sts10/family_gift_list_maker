use family_gift_list_maker::*;

fn main() {
    println!("\nEnter the file path of the CSV file with the family names");
    let names_file_path = get_file_path();
    let names: Vec<Vec<String>> = read_csv(&names_file_path);
    let names = sort_families(names);

    println!("\nOptionally, enter file path for a text list of previous years' giving\n(Hit <enter> if you do not want to enter such a file)");
    let previous_years_file_path = get_file_path();
    let previous_years_giving: Vec<String> = if previous_years_file_path.is_empty() {
        [].to_vec()
    } else {
        read_by_line(&previous_years_file_path).unwrap()
    };

    println!("\nOptionally, enter file path for a text list of special requests\n(Hit <enter> if you do not want to enter such a file)");
    let special_requests_file_path = get_file_path();
    let special_requests: Vec<String> = if special_requests_file_path.is_empty() {
        [].to_vec()
    } else {
        read_by_line(&special_requests_file_path).unwrap()
    };

    println!("\n");

    // loop until we get a good solution
    loop {
        match find_gift_givers(&names, &previous_years_giving, &special_requests) {
            Some(mut pairs) => {
                // sort list alphabetically to cover evidence of special requests
                pairs.sort();
                for pair in pairs {
                    println!("{} gives to {}", pair.0, pair.1);
                }
                break;
            }
            None => {
                println!("\n------------------");
                println!("Got a bad solution\nGoing to try again");
                println!("------------------\n");
                continue;
            }
        };
    }
    println!("\n------------------");
    println!("Done!");
}
