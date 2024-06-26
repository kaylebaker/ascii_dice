use std::io;
use std::collections::HashMap;
use rand::Rng;
use std::thread;
use std::time::Duration;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{stdout, Write};
use anyhow::{Result, Error};

// Define an enum for different pip patterns on a dice face
#[derive(Clone, Copy)]
enum Pips {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl Pips {
    // Method to convert pip pattern to an array of strings for printing
    fn as_array(&self) -> &'static [&'static str] {
        match self {
            Pips::One => &["┌─────────┐", "│         │", "│    ●    │", "│         │", "└─────────┘"],
            Pips::Two => &["┌─────────┐", "│    ●    │", "│         │", "│    ●    │", "└─────────┘"],
            Pips::Three => &["┌─────────┐", "│ ●       │", "│    ●    │", "│       ● │", "└─────────┘"],
            Pips::Four => &["┌─────────┐", "│ ●     ● │", "│         │", "│ ●     ● │", "└─────────┘"],
            Pips::Five => &["┌─────────┐", "│ ●     ● │", "│    ●    │", "│ ●     ● │", "└─────────┘"],
            Pips::Six => &["┌─────────┐", "│ ●     ● │", "│ ●     ● │", "│ ●     ● │", "└─────────┘"],
        }
    }
}

// Define a struct for a single dice
struct Dice {
    current_face: Pips,
}

impl Dice {
    // Method to roll the dice and update its face
    fn roll_dice(&mut self) {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1..=6);
        match random_number {
            1 => self.current_face = Pips::One,
            2 => self.current_face = Pips::Two,
            3 => self.current_face = Pips::Three,
            4 => self.current_face = Pips::Four,
            5 => self.current_face = Pips::Five,
            6 => self.current_face = Pips::Six,
            _ => unreachable!(),
        };
    }

    // Method to print the current face of the dice
    fn _print_face(&self) {
        for line in self.current_face.as_array() {
            println!("{}", line);
        }
    }
}

// Define a struct for a cup of dice
struct DiceCup {
    dice: Vec<Dice>,
    hm: HashMap<usize, String>,
}

impl DiceCup {
    // Method to fill the cup with a specified number of dice
    fn fill_cup(&mut self, number_of_dice: usize) {
        for _ in 0..number_of_dice {
            let d6 = Dice {
                current_face: Pips::One,
            };
            self.dice.push(d6);
        }
    }
    
    // Method to roll all dice in the cup
    fn roll_cup(&mut self) {
        for dice in self.dice.iter_mut() {
            dice.roll_dice();
        }
    }

    // Method to generate and display output for all dice in the cup
    fn print_roll(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        let _ = stdout.execute(cursor::Hide);

        for dice in self.dice.iter_mut() {
            for i in 0..5 {
                let mut entry = if self.hm.contains_key(&i) {
                    self.hm.get(&i).cloned().unwrap()
                } else {
                    String::from("\t")
                };
                entry.push_str("  ");
                entry.push_str(dice.current_face.as_array()[i]);
                self.hm.insert(i, entry);
            }


            // Move the cursor up 5 lines
            stdout.execute(cursor::MoveUp(5))?;

            // Clear the screen from cursor to the end
            stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;

            for i in 0..self.hm.len() {
                writeln!(stdout, "{}", self.hm[&i])?;
            }
            thread::sleep(Duration::from_millis(250));
        }
        let _ = stdout.execute(cursor::Show);
        Ok(())

    }

    fn save_current_faces(&self, vector: &mut Vec<Vec<i32>>) {
        let mut faces: Vec<i32> = Vec::new();
        for dice in &self.dice {
            faces.push(dice.current_face as i32);
        }
        vector.push(faces);
    }

}


fn main() {
    let mut stdout = stdout();
    let mut user_input = String::new();
    let mut roll_count = 1;
    let mut roll_results: Vec<Vec<i32>> = Vec::new();

    println!("\n Roll #");
    println!("--------------------------------------------------------------------------------------------------------------------------------\n");

    loop {
        user_input.clear();

        println!("--------------------------------------------------------------------------------------------------------------------------------");
        println!("\nHow many dice would you like to roll?");
        writeln!(stdout, ">").unwrap();
        stdout.execute(cursor::MoveUp(1)).unwrap();
        stdout.execute(cursor::MoveRight(2)).unwrap();
        io::stdin().read_line(&mut user_input).expect("failed to read line");
        let user_input = user_input.trim();

        // Check if the input is a valid number, else break the loop
        let dice_qty: usize = if user_input.chars().all(|c| c.is_numeric()) {
            user_input.parse().expect("Not a valid number")
        } else {
            break;
        };

        // Create a new DiceCup and fill it with dice
        let mut dicecup = DiceCup {
            dice: Vec::new(),
            hm: HashMap::new(),
        };
        dicecup.fill_cup(dice_qty);

        dicecup.roll_cup();
        let _ = dicecup.print_roll();
        dicecup.save_current_faces(&mut roll_results);

        let _ = stdout.execute(cursor::MoveUp(3));
        let _ = stdout.execute(cursor::MoveToColumn(0));
        let _ = writeln!(stdout, "  {}", roll_count);
        roll_count += 1;
        let _ = stdout.execute(cursor::MoveDown(3));
    }

    // The code below gathers stats of all the rolls and prints them to the console

    let mut total_number_of_dice_rolled = 0;
    let mut total_of_rolls_all = 0;
    let mut highest_roll_all = 0;
    let mut lowest_roll_all = 99;
    let mut per_roll_stats: Vec<HashMap<&str, i32>> = vec![];
    
    println!("\nHere are your results...");
    println!("\nROLL\tSTATS");
    println!("-----------------------------------------------");
    
    for (i, vec) in roll_results.iter().enumerate() {
        let mut temp_hm: HashMap<&str, i32> = HashMap::new();
        let mut highest = 0;
        let mut lowest = 99;
        let mut total = 0;

        for x in vec {
            highest = if x > &highest { *x } else { highest };
            lowest = if x < &lowest { *x } else { lowest };
            total = total + x;
        }
        temp_hm.insert("total", total);
        temp_hm.insert("highest", highest);
        temp_hm.insert("lowest", lowest);

        per_roll_stats.push(temp_hm);
        
        total_number_of_dice_rolled += vec.len();
        total_of_rolls_all += total;
        highest_roll_all = if highest > highest_roll_all { highest } else { highest_roll_all };
        lowest_roll_all = if lowest < lowest_roll_all { lowest } else { lowest_roll_all };
        
        println!("\n#{}\t{:?}", i + 1, vec);
        println!("\tYou rolled {} dice", vec.len());
        println!("\tThe sum of these {} dice is {}", vec.len(), total);
        println!("\tThe highest dice for this roll was a {}", highest);
        println!("\tThe lowest dice for this roll was a {}", lowest);
    }
    println!("\n-----------------------------------------------");
    println!("You rolled a total of {} dice!", total_number_of_dice_rolled);
    println!("The total sum of all dice rolled is {}", total_of_rolls_all);
    println!("The highest dice rolled of all dice was a {}", highest_roll_all);
    println!("The lowest dice rolled of all dice was a {}", lowest_roll_all);
}





// TESTS //
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_dice() {
        // Determine number of test runs
        let num_runs = 100;

        // Repeat test number of times as per num_runs
        for _ in 0..num_runs {

            // Create a dice instance
            let mut dice = Dice { current_face: Pips::One };

            // Roll the dice
            dice.roll_dice();

            // Check that the dice face is within valid range of Pip patterns
            match dice.current_face {
                Pips::One | Pips::Two | Pips::Three | Pips::Four | Pips::Five | Pips::Six => {
                    // Test passed if the face is valid
                    assert!(true);
                }
                _ => {
                    // Test failed if the face is not valid
                    assert!(false, "Invalid dice face after rolling");
                }
            }
        }
    }
}
