use crate::custom_error::CSVError;

#[derive(Debug)]
pub struct CSVData {
    pub data: Vec<Vec<String>>, // Holds the CSV data
    dimensions: (Row, Column),  // Dimensions (rows, columns)
}

impl CSVData {
    pub fn read_csv(path: &str) -> Result<Self, CSVError> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)
            .map_err(|e| match e.kind() {
                csv::ErrorKind::Io(io_err) if io_err.kind() == std::io::ErrorKind::NotFound => {
                    CSVError::FileNotFound(path.to_owned())
                }
                _ => CSVError::Other(Box::new(e)),
            })?;
        let mut data: Vec<Vec<String>> = Vec::new();

        for result in rdr.records() {
            let record = match result {
                Ok(r) => r,
                Err(e) => return Err(CSVError::Other(Box::new(e))),
            };
            data.push(record.iter().map(|s| s.to_string()).collect());
        }

        let dimensions = (
            Row(data.len()),
            Column(data.first().map_or(0, |row| row.len())),
        );
        Ok(CSVData { data, dimensions })
    }

    pub fn display_data(&self) {
        println!("\n");
        for row in &self.data {
            println!("{}", row.join(","))
        }
    }
    pub fn paginate(
        &self,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> Result<Vec<Vec<String>>, CSVError> {
        let mut display_data: Vec<Vec<String>> = Vec::new();

        for row in start_row..=end_row {
            let start_idx = if row == start_row { start_col } else { 0 };
            let end_idx = if row == end_row {
                end_col + 1
            } else {
                self.dimensions.1 .0
            };

            let row_data: Vec<String> = self.data[row][start_idx..end_idx].to_vec();
            display_data.push(row_data);
        }

        Ok(display_data)
    }
    pub fn delete_cell(&mut self, row: usize, col: usize) -> Result<(), CSVError> {
        self.data[row][col] = "_".to_string();
        Ok(())
    }

    pub fn modify_cell(&mut self, row: usize, col: usize, new_value: &str) -> Result<(), CSVError> {
        self.data[row][col] = new_value.to_string();
        Ok(())
    }

    pub fn save_to_csv(&self, file_name: &str) -> Result<(), CSVError> {
        let current_dir = std::env::current_dir()?;
        let parent_dir =
            current_dir
                .parent()
                .ok_or(CSVError::Other(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Parent directory not found",
                ))))?;

        // Create the full file path by joining the parent directory, file name, and ".csv" extension
        let file_path = parent_dir.join(file_name.to_string() + ".csv");

        let mut writer = csv::Writer::from_path(file_path)?;

        for row in &self.data {
            writer.write_record(row)?;
        }

        writer.flush()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Row(usize);

#[derive(Debug, Clone, Copy)]
pub struct Column(usize);
