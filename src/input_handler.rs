use std::{
    io::{self, Write},
    thread::sleep,
    time,
};

use crate::{csv_data::*, custom_error::CSVError};

type PaginationRange = ((usize, usize), (usize, usize));
pub trait InputValidator {
    fn validate(input: usize) -> Result<(), CSVError>;
    fn description() -> &'static str;
}

impl InputValidator for Row {
    fn validate(input: usize) -> Result<(), CSVError> {
        if (1..=7).contains(&input) {
            Ok(())
        } else {
            Err(CSVError::ValidationError(
                "Row out of bounds (1-7)".to_owned(),
            ))
        }
    }

    fn description() -> &'static str {
        "Row numbers range from 1 to 7"
    }
}

impl InputValidator for Column {
    fn validate(input: usize) -> Result<(), CSVError> {
        if (1..=6).contains(&input) {
            Ok(())
        } else {
            Err(CSVError::ValidationError(
                "Column out of bounds (1-6)".to_owned(),
            ))
        }
    }

    fn description() -> &'static str {
        "Column numbers range between 1 and 6"
    }
}

pub fn read_menu_selection() -> Result<String, CSVError> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter again");
    }
    let input = buffer.trim().to_owned();
    if input.is_empty() {
        Err(CSVError::InputError("Empty string".to_owned()))
    } else {
        Ok(input)
    }
}

fn get_pagination_range() -> Result<PaginationRange, CSVError> {
    let (start_row, start_column) = get_validated_row_column(
        "\nEnter starting row index.",
        "Enter starting column index.",
    )?;
    let (end_row, end_column) =
        get_validated_row_column("\nEnter end row index.", "Enter end column index.")?;

    if start_row <= end_row && (start_row < end_row || start_column <= end_column) {
        Ok((
            (start_row - 1, start_column - 1),
            (end_row - 1, end_column - 1),
        ))
    } else {
        Err(CSVError::ValidationError("Invalid range: ensure that end row/column is greater than or equal to start row/column.".to_owned()))
    }
}

fn get_single_cell() -> Result<(usize, usize), CSVError> {
    get_validated_row_column("\nEnter row index.", "\nEnter column index.")
        .map(|(row, col)| (row - 1, col - 1)) // Adjust for zero-based indexing
}

fn get_validated_row_column(
    prompt_row: &str,
    prompt_column: &str,
) -> Result<(usize, usize), CSVError> {
    let row = handle_input::<Row>(prompt_row)?;
    let column = handle_input::<Column>(prompt_column)?;
    Ok((row, column))
}

fn retrieve_input<T: InputValidator>() -> Result<usize, CSVError> {
    let mut buffer = String::new();
    println!("{}:\n", T::description());
    io::stdout().flush().unwrap();

    if io::stdin().read_line(&mut buffer).is_err() {
        return Err(CSVError::InputError("Failed to read input".to_owned()));
    }

    let input = buffer.trim();
    if input == "menu" {
        return Err(CSVError::MenuReset);
    }

    let num = input.parse::<usize>().map_err(|_| {
        CSVError::InputError("Invalid input. Please enter a valid number.".to_owned())
    })?;

    T::validate(num)?;
    Ok(num)
}

fn handle_input<T: InputValidator>(prompt: &str) -> Result<usize, CSVError> {
    loop {
        println!("{}", prompt);
        match retrieve_input::<T>() {
            Ok(index) => return Ok(index),
            Err(CSVError::MenuReset) => return Err(CSVError::MenuReset),
            Err(_) => println!("Invalid input, please try again."),
        }
    }
}

pub fn paginate_data(data: &CSVData) -> Result<(), CSVError> {
    let ((start_row, start_col), (end_row, end_col)) = get_pagination_range()?;
    let display_data = data.paginate(start_row, start_col, end_row, end_col)?;

    for row in display_data {
        println!("{}", row.join(", "));
    }
    eprintln!("Returning to main menu...");
    sleep(time::Duration::from_secs(2));

    Ok(())
}

pub fn delete_cell_in_data(data: &mut CSVData) -> Result<(), CSVError> {
    let (row, col) = get_single_cell()?;
    println!(
        "\nAre you sure you want to delete this cell {:?}? [y/n]: ",
        data.data[row][col]
    );
    let confirm = confirm_action()?;
    if confirm {
        let result = data.delete_cell(row, col);
        println!("Succesfully deleted cell.");
        eprintln!("Returning to main menu...\n");
        sleep(time::Duration::from_secs(2));
        result
    } else {
        println!("Operation cancelled.");
        Err(CSVError::MenuReset)
    }
}

pub fn modify_cell_in_data(data: &mut CSVData) -> Result<(), CSVError> {
    let (row, col) = get_single_cell()?;
    println!(
        "\nAre you sure you want to modify this cell {:?}? [y/n]: ",
        data.data[row][col]
    );
    let confirm = confirm_action()?;
    if confirm {
        println!("Enter the new value for the cell:");
        let mut new_value = String::new();
        io::stdin()
            .read_line(&mut new_value)
            .map_err(|_| CSVError::InputError("Failed to read input".to_owned()))?;
        let result = data.modify_cell(row, col, new_value.trim());
        println!("Succesfully modified cell.");
        eprintln!("Returning to main menu...\n");
        sleep(time::Duration::from_secs(2));
        result
    } else {
        println!("Operation cancelled.");
        Err(CSVError::MenuReset)
    }
}

pub fn save_to_csv_file(data: &CSVData) -> Result<(), CSVError> {
    println!("Enter the name for the new or existing CSV file:");
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .map_err(|_| CSVError::InputError("Failed to read input".to_owned()))?;

    // Trim the newline character from the input
    file_name = file_name.trim().to_string();

    data.save_to_csv(&file_name)?;
    println!("Data saved to CSV file: {}", file_name);
    eprintln!("Returning to main menu...\n");
    sleep(time::Duration::from_secs(2));
    Ok(())
}
fn confirm_action() -> Result<bool, CSVError> {
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|_| CSVError::InputError("Failed to read input".to_owned()))?;
    Ok(buffer.trim().eq_ignore_ascii_case("y"))
}
