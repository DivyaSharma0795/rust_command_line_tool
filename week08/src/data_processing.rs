use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Student {
    name: String,
    marks: i32,
}

pub fn ingest_data(filename: &str) -> io::Result<Vec<Student>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines().skip(1) { // Skip the header line
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let student = Student {
                name: parts[1].to_string(),
                marks: parts[2].parse().unwrap_or(0),
            };
            data.push(student);
        }
    }

    Ok(data)
}

pub fn process_data(data: &[Student]) -> Vec<String> {
    data.iter()
        .map(|student| format!("{} received {} marks", student.name, student.marks))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingest_data() {
        let data = ingest_data("test.csv").unwrap();
        assert_eq!(data.len(), 3); // Check if we have 3 students
        assert_eq!(data[0].name, "John");
        assert_eq!(data[0].marks, 100);
        assert_eq!(data[1].name, "Smith");
        assert_eq!(data[1].marks, 90);
        assert_eq!(data[2].name, "David");
        assert_eq!(data[2].marks, 80);
    }

    #[test]
    fn test_process_data() {
        let data = vec![
            Student { name: "John".to_string(), marks: 100 },
            Student { name: "Smith".to_string(), marks: 90 },
            Student { name: "David".to_string(), marks: 80 },
        ];
        let processed_data = process_data(&data);
        assert_eq!(processed_data, vec![
            "John received 100 marks",
            "Smith received 90 marks",
            "David received 80 marks",
        ]);
    }
}