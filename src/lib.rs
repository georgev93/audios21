//! Documentation

mod audio;
use crate::audio::{StreamSink, StreamSource};

mod measurement_kit;
use measurement_kit::MeasurementTypes;

struct MeasurementSuite {
    input_stream: Option<StreamSource>,
    output_stream: Option<StreamSink>,
    measurements: Vec<MeasurementTypes>,
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
    fn request_measurement(&mut self, measurement: Box<MeasurementTypes>);
}

impl MeasurementSuiteAPI for MeasurementSuite {
    fn request_measurement(&mut self, measurement: Box<MeasurementTypes>) {
        self.measurements.push(*measurement);
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
    fn mytest() {
        let mut my_suite = MeasurementSuite::new();
        let my_measurement =
            measurement_kit::MeasurementTypes::S21(measurement_kit::S21::new("test"));

        my_suite.request_measurement(Box::new(my_measurement));

         assert_eq!(my_suite.measurements.len(), 1);
         // assert_eq!(my_suite.measurements[0].name(), "test");
    }
}
