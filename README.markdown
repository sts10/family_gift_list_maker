# Family Gift (Secret Santa) List Maker

**Note**: A newer version of this program is now called [fgift](https://github.com/sts10/fgift).

Takes an input of a CSV file, describing an extended family, with each line an immediate family. Outputs a list of who should give to who, ensuring that no one gives to anyone in their immediate family, but otherwise selecting randomly.

Obviously you could substitute the concept of "families" for small teams at your work.

## Example CSV

Make a CSV where each row is an immediate family. 

For example, a CSV like this...

```csv
Names,,,,
Claire,Phil,Haley,Alex,Luke
Cameron,Mitchell,Lily,,
Jay,Gloria,Manny,,
```

Could give you an output like this:

```
Claire gives to Cameron
Phil gives to Mitchell
Haley gives to Manny
Alex gives to Jay
Luke gives to Gloria
Baby gives to Lily
Jay gives to Haley
Gloria gives to Alex
Manny gives to Baby
Cameron gives to Phil
Mitchell gives to Luke
Lily gives to Claire
```

As you can see, for example, Claire does not give to Phil, Haley, Alex, or Luke, since they are in the same immediate family (this directive is established in the input CSV -- they're all in the same row). 

## How to make your own list

[Install Rust](https://www.rust-lang.org/en-US/install.html). Clone down this directory, `cd` into it, then run `cargo run`. You'll be prompted to enter the file location of your CSV file.

`test-names.csv` provides a template for this CSV input file-- basically each immediate family goes on its own line, with the first row being a title (it doesn't matter what the title is).

Optionally, you can enter a text file listing who gave to who in previous years. This file should look just like the output text (i.e. "Claire gives to Cameron").

You can also optionally input a text file containing "special requests," i.e. people who _must_ give to another given person. This file should look just like the output text (i.e. "Claire gives to Cameron").

Once your CSV file (and other optional files) is ready, run `cargo run`, enter the requested file location(s), and see who's giving to who!

## Notes about the CSV input file

Given the way I have the Rust code reading the CSV, it will ignore the first line of the CSV file. So just keep that as a generic title, like "Names".

Generally I'd recommend creating and editing the CSV files in a spreadsheet editor like Microsoft Excel or [LibreOffice](https://www.libreoffice.org/) Calc, rather than a text editor like Vim or Sublime Text.

## Notes on "Randomness"

Are the results of this program truly random? Or in other words, is it somehow bias in choosing who gives to whom? 

Of course the assignments are affected by the no-immediate-family-assignments rule, but outside of that, **I'm trying to make the assignment logic as "random" as possible** while still following the no-immediate-family-assignments rule.

(One thing that got me wondering about this -- and may offer clues? -- was [this video](https://www.youtube.com/watch?v=5kC5k5QBqcc).)

## Testing for bias: A statistics problem

To see if the program indeed had such a bias, I decided to try to write a series of tests in which I aim to perform a [Pearson chi-squared test](https://en.wikipedia.org/wiki/Pearson%27s_chi-squared_test). The tests and their associated helper function are currently in `src/lib.rs` (the first one is called `chi_squared_test_claire`). Note that I am not a statistician and relied mostly on my high school stats knowledge in making this choice... so if you have a better idea of how to test for this type of bias please create a GitHub issue or pull request!

### The test I wrote

The tests I wrote all use this helper function (located in `src/lib.rs`) to do the chi-squared test.

```rust
    fn individual_giver_chi_test(giver_name: String, upper_tail_critical: f64) -> bool {
        // run 1000 trials to get a Vector of observed values
        let mut observed_receivers_hashmap: HashMap<String, usize> = HashMap::new();

        for _ in 0..1000 {
            let pairs = make_a_list_no_requests_or_previous_years();
            for pair in pairs {
                if pair.0 == giver_name {
                    observed_receivers_hashmap
                        .entry(pair.1)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }
        // this won't find cases where Claire _never_ gives to a legitimate receiver... _probably_ not a huge deal

        // convert the Hashmap to a Vector to make it easier to iterate through
        let observed_receivers_vec: Vec<(&String, &usize)> =
            observed_receivers_hashmap.iter().collect();

        // now calculate the chi-squared statistic
        let mut chi_squared_statistic: f64 = 0.0;
        println!("For {}... ", giver_name);
        for (_n, observed_receiver_name_and_count) in observed_receivers_vec.iter().enumerate() {
            let receiver_name = observed_receiver_name_and_count.0;
            let observed_count = observed_receiver_name_and_count.1;

            let expected_count: f64 =
                1000.0 / (look_up_number_of_potential_receivers(&giver_name) as f64);

            println!(
                "We expected {} to give to {} {} times out of 1000; Observed: {} times out of 1000",
                giver_name, receiver_name, expected_count, observed_count
            );

            chi_squared_statistic = chi_squared_statistic
                + (*observed_count as f64 - expected_count as f64).powf(2.0)
                    / expected_count as f64;
        }

        println!(
            "For {}, found a chi squared of {}",
            giver_name, chi_squared_statistic
        );

        chi_squared_statistic < upper_tail_critical
    }
```

As the code stands, the tests pass when testing "givers" in the _largest_ family (Claire and Phil are examples, but **fail** for givers in the two _smaller_ families (Cameron and Manny are the two I test currently). 

### How the two tests fail, currently

Here's the printed output for the failed Cameron test:

```text
---- integration_tests::chi_squared_test_cameron stdout ----
For Cameron... 
We expected Cameron to give to Haley 125 times out of 1000; Observed: 145 times out of 1000
We expected Cameron to give to Manny 125 times out of 1000; Observed: 52 times out of 1000
We expected Cameron to give to Phil 125 times out of 1000; Observed: 156 times out of 1000
We expected Cameron to give to Jay 125 times out of 1000; Observed: 61 times out of 1000
We expected Cameron to give to Alex 125 times out of 1000; Observed: 172 times out of 1000
We expected Cameron to give to Luke 125 times out of 1000; Observed: 181 times out of 1000
We expected Cameron to give to Gloria 125 times out of 1000; Observed: 55 times out of 1000
We expected Cameron to give to Claire 125 times out of 1000; Observed: 178 times out of 1000
For Cameron, found a chi squared of 190.72
```

As you can see, Cameron is assigned receivers in the large family of 5 (Haley, Phil, Alex, Luke, and Claire) more often than the other, smaller family of three (Manny, Jay, and Gloria). This happens even when the inputed family list is shuffled by family and within each family (see `fn shuffle_families` in `src/lib.rs`).

Here's the printout for the failed Manny test, which has a similar bias as the Cameron test:

```text
---- integration_tests::chi_squared_test_manny stdout ----
For Manny... 
We expected Manny to give to Luke 125 times out of 1000; Observed: 170 times out of 1000
We expected Manny to give to Lily 125 times out of 1000; Observed: 56 times out of 1000
We expected Manny to give to Haley 125 times out of 1000; Observed: 149 times out of 1000
We expected Manny to give to Claire 125 times out of 1000; Observed: 166 times out of 1000
We expected Manny to give to Phil 125 times out of 1000; Observed: 176 times out of 1000
We expected Manny to give to Cameron 125 times out of 1000; Observed: 61 times out of 1000
We expected Manny to give to Alex 125 times out of 1000; Observed: 144 times out of 1000
We expected Manny to give to Mitchell 125 times out of 1000; Observed: 78 times out of 1000
For Manny, found a chi squared of 146.48
```

I can't figure out how to get these two tests to pass. Or if that's even possible.

## Attempts to pass my bias tests

In an effort to make these two tests pass (as written), I had the code shuffle both the order of families and the order of names within the families for each trial (see: `fn shuffle_families` in `src/lib.rs`). I also tried assigning receivers to the two small families first and _then_ assigning members of the large family. Neither made the tests pass. 

Given that one of the requirements of the system is that you cannot be assigned to give to a member of your own immediate family (and that nuclear families will almost certainly be of different sizes), I'm not sure if it's even possible to make the chi-squared tests pass as I've written them. 

That said... 

### Other ideas for passing my bias tests that I haven't tried

One idea, though: the program should do a full restart whenever a "bad" choice (i.e. picks herself, picks someone in her family, or someone she has given to in the past) is made randomly. That way, when we finally get a "good" run-through, we're more assured that every selection was freely random. 
