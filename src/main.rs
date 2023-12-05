mod csv_data;
mod custom_error;
mod input_handler;

use core::time;
use std::thread::sleep;

use csv_data::CSVData;
use custom_error::CSVError;
use input_handler::{
    delete_cell_in_data, modify_cell_in_data, paginate_data, read_menu_selection, save_to_csv_file,
};

const CSVFILE: &str = "../testdata.csv";

enum MainMenu {
    DisplayFile,
    PaginateFile,
    DeleteField,
    UpdateField,
    WriteFile,
    Quit,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::DisplayFile),
            "2" => Some(Self::PaginateFile),
            "3" => Some(Self::DeleteField),
            "4" => Some(Self::UpdateField),
            "5" => Some(Self::WriteFile),
            "q" | "quit" => Some(Self::Quit),
            _ => None,
        }
    }
    fn show_menu() {
        println!("\n == CSV Manager ==\n");
        println!("1. Display Entire File");
        println!("2. Paginate File");
        println!("3. Delete Field");
        println!("4. Update Field");
        println!("5. Create New CSV File\n");
        println!("Please enter your selection using the corresponding menu number only or enter \"q\" or \"quit\" to exit:");
    }
}

fn run_prog() -> Result<(), CSVError> {
    let mut data = CSVData::read_csv(CSVFILE)?;

    loop {
        MainMenu::show_menu();
        let input = read_menu_selection()?;
        match MainMenu::from_str(&input) {
            Some(MainMenu::DisplayFile) => data.display_data(),
            Some(MainMenu::PaginateFile) => match paginate_data(&data) {
                Ok(_) => (),
                Err(CSVError::MenuReset) => {
                    eprintln!("Returning to main menu...");
                    sleep(time::Duration::from_secs(2));
                    continue;
                }
                Err(e) => eprintln!("Error: {}", e),
            },
            Some(MainMenu::DeleteField) => match delete_cell_in_data(&mut data) {
                Ok(_) => (),
                Err(CSVError::MenuReset) => {
                    eprintln!("Returning to main menu...");
                    sleep(time::Duration::from_secs(2));
                    continue;
                }
                Err(e) => eprintln!("Error: {}", e),
            },
            Some(MainMenu::UpdateField) => match modify_cell_in_data(&mut data) {
                Ok(_) => (),
                Err(CSVError::MenuReset) => {
                    eprintln!("Returning to main menu...");
                    sleep(time::Duration::from_secs(2));
                    continue;
                }
                Err(e) => eprintln!("Error: {}", e),
            },
            Some(MainMenu::WriteFile) => match save_to_csv_file(&data) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e),
            },
            Some(MainMenu::Quit) => {
                println!("Exiting the program.");
                break; // Exit the loop and program
            }
            None => {
                println!("Unrecognized command. Please try again.");
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run_prog() {
        eprintln!("Error: {}", e);
    }
}
