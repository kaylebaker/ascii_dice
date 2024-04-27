use std::io;
use std::collections::HashMap;
use rand::Rng;
use std::thread;
use std::time::Duration;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{stdout, Write};
use anyhow::{Result, Error};

enum Pips {
    One,
    Two,
    Three,
    Four,
    Five,
    Six, 
}

impl Pips {
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

struct Dice {
    current_face: Pips,
}

impl Dice {
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

    fn _print_dice(&self) {
        for line in self.current_face.as_array() {
            println!("{}", line);
        }
    }
}

struct DiceCup {
    dice: Vec<Dice>,
    hm: HashMap<usize, String>,
}

impl DiceCup {
    fn fill_cup(&mut self, number_of_dice: u8) {
        for _ in 0..number_of_dice {
            let d6 = Dice {
                current_face: Pips::One,
            };
            self.dice.push(d6);
        }
    }

    fn generate_output(&mut self) {
        for dice in self.dice.iter_mut() {

            for i in 0..5 {
                let mut entry = if self.hm.contains_key(&i) { self.hm.get(&i).cloned().unwrap() } else { String::from("\t") };
                entry.push_str("  ");
                entry.push_str(dice.current_face.as_array()[i]);
                self.hm.insert(i, entry);
            }

        }

        let _ = self.refresh_output();
    }

    fn refresh_output(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();

        // Move the cursor up 5 lines
        stdout.execute(cursor::MoveUp(5))?;

        // Clear the screen from cursor to the end
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;

        for i in 0..self.hm.len() {
            writeln!(stdout, "{}", self.hm[&i])?;
        }

        Ok(())
    }

    fn roll_cup(&mut self) {
        for dice in self.dice.iter_mut() {
            dice.roll_dice();
        }
    }
}


fn main() {

    let mut stdout = stdout();
    let mut user_input = String::new();
    let mut roll_count = 1;

    println!("\n Roll #");
    println!("----------------------------------------------------------------------------------------------------------------------------------------------------\n");
    loop {
        user_input.clear();

        println!("----------------------------------------------------------------------------------------------------------------------------------------------------");
        println!("\nHow many dice would you like to roll?");
        writeln!(stdout, ">").unwrap();
        stdout.execute(cursor::MoveUp(1)).unwrap();
        stdout.execute(cursor::MoveRight(2)).unwrap();
        io::stdin().read_line(&mut user_input).expect("failed to read line");
        let user_input = user_input.trim();

        let dice_qty: u8 = if user_input.chars().all(|c| c.is_numeric()) { user_input.parse().expect("Not a valid number") } else { break };
        
        let mut dicecup = DiceCup {
            dice: Vec::new(),
            hm: HashMap::new(),
        };
        dicecup.fill_cup(1);

        for _ in 0..dice_qty {
            dicecup.roll_cup();
            thread::sleep(Duration::from_millis(150));
            dicecup.generate_output();
        }
        let _ = stdout.execute(cursor::MoveUp(3));
        let _ = stdout.execute(cursor::MoveToColumn(0));
        let _ = writeln!(stdout, "  {}", roll_count);
        roll_count += 1;
        let _ = stdout.execute(cursor::MoveDown(3));
}
}