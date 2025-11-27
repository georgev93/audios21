use crate::measurement_kit::{Measurement, MeasurementCommon, TestOutcome, TestResult};

pub struct S21Data {
    freq: Vec<u32>,
    amp: Vec<f64>,
}

pub struct S21 {
    pub base: MeasurementCommon<S21Data>,
}

impl S21 {
    pub fn new(name: &str) -> Self {
        Self {
            base: MeasurementCommon {
                name: name.to_string(),
                ran: false,
                data: None,
                expected_result: None,
            },
        }
    }
}

impl Measurement for S21 {
    type MeasurementData = S21Data;

    fn name(&self) -> &str {
        &self.base.name
    }

    fn run(&self) -> TestResult {
        println!("Pretending to run an S21 test");
        let mut collected_data = S21Data {
            freq: Vec::new(),
            amp: Vec::new(),
        };
        collected_data.freq.push(1000);
        collected_data.amp.push(1.0);
        if self.base.expected_result.is_some() {
            Ok(TestOutcome::Passed)
        } else {
            Ok(TestOutcome::Ran)
        }
    }

    fn set_expectation(&mut self, data: Self::MeasurementData) {
        self.base.expected_result = Some(data);
    }

    fn get_data(&self) -> Option<&Self::MeasurementData> {
        self.base.data.as_ref()
    }
}
