//! Documentation

mod audio;
use crate::audio::{StreamSink, StreamSource};

mod measurement_kit;
use measurement_kit::Measurement;

struct MeasurementSuite {
    input_stream: Option<StreamSource>,
    output_stream: Option<StreamSink>,
    measurements: Vec<Box<dyn measurement_kit::Measurement>>,
}

struct MeasurementSuiteConfig {
    input: Option<String>,
    output: Option<String>,
    sample_rate: u32,
}

/// API for the measurement suite
pub trait MeasurementSuiteAPI {
    /// Add a Measurement to the vector of measurements to be performed
    ///
    /// * `measurement`: The measurement to be added to the suite
    fn request_measurement(&mut self, measurement: Box<dyn Measurement>);
}

impl MeasurementSuiteAPI for MeasurementSuite {
    fn request_measurement(&mut self, measurement: Box<dyn Measurement>) {
        self.measurements.push(measurement);
    }
}

impl MeasurementSuite {
    fn new() -> Self {
        MeasurementSuite {
            input_stream: None,
            output_stream: None,
            measurements: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requesting_measurements() {
        let mut my_suite = MeasurementSuite::new();
        let my_measurement_1 = measurement_kit::s21::S21::new("1");
        let my_measurement_2 = measurement_kit::s21::S21::new("2");

        assert_eq!(my_suite.measurements.len(), 0);

        my_suite.request_measurement(Box::new(my_measurement_1));
        assert_eq!(my_suite.measurements.len(), 1);
        assert_eq!(my_suite.measurements[0].name(), "1");

        my_suite.request_measurement(Box::new(my_measurement_2));
        assert_eq!(my_suite.measurements.len(), 2);
        assert_eq!(my_suite.measurements[0].name(), "1");
        assert_eq!(my_suite.measurements[1].name(), "2");

    }
}
