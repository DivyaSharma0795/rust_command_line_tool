// data_processing_test.rs
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