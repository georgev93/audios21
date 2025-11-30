use crate::measurement_kit::{Measurement, TestOutcome, TestResult};

#[derive(Debug, PartialEq)]
pub struct S21Data {
    freq: Vec<u32>,
    amp: Vec<f64>,
}

#[derive(Debug, PartialEq)]
pub struct S21 {
    /// Name of the measurement
    name: String,

    /// Whether or not the measurement has been taken
    ran: bool,

    /// The data collected by the measurement
    data: Option<S21Data>,

    /// The expected result (if this test is pass/fail)
    expected_result: Option<S21Data>,
}

impl S21 {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ran: false,
            data: None,
            expected_result: None,
        }
    }
}

impl Measurement for S21 {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestResult {
        println!("Pretending to run an S21 test");
        let mut collected_data = S21Data {
            freq: Vec::new(),
            amp: Vec::new(),
        };
        collected_data.freq.push(1000);
        collected_data.amp.push(1.0);

        if self.expected_result.is_some() {
            Ok(TestOutcome::Passed)
        } else {
            Ok(TestOutcome::Ran)
        }
    }

    fn calibrate(&self) -> Result<(), std::io::Error>  {
        Ok(())
    }
}
