//! q

//! Documentation

mod audio;
use crate::audio::{AudioDriver, AudioDriverImpl, StreamSink, StreamSource};

mod measurement_kit;
use measurement_kit::Measurement;

/// Possible Audio Selections for Source/Sink
#[derive(Debug)]
pub enum StreamChoice {
    /// Uses the system's default device
    DefaultDevice(),

    /// Uses a WAV file as a source or sink
    WAV(String),

    /// Select a device for source or sink by matching a string
    Device(String),
}

/// Configures and runs a series of audio measurements and tests.
///
/// * `input_stream`: The input audio stream to use
/// * `output_stream`: The output audio stream to use
/// * `measurements`: A vector of measurements to take
pub struct MeasurementSuite<A: AudioDriver> {
    audio_driver: A,
    input_stream: Option<StreamSource>,
    output_stream: Option<StreamSink>,
    measurements: Vec<Box<dyn measurement_kit::Measurement>>,
}

struct MeasurementSuiteConfig {
    input: Option<String>,
    output: Option<String>,
    sample_rate: u32,
}

impl MeasurementSuite<AudioDriverImpl> {
    /// New
    #[must_use]
    pub fn new() -> Self {
        Self {
            audio_driver: AudioDriverImpl::new(),
            input_stream: None,
            output_stream: None,
            measurements: Vec::new(),
        }
    }
}

impl<A: AudioDriver> MeasurementSuite<A> {
    /// Constructor to allow for dependency injection of the ``AudioDriver`` object (mainly for
    /// testing)
    ///
    /// * `audio_driver`: Which driver to use for audio (could be a mock)
    pub fn with_audio_driver(audio_driver: A) -> Self {
        Self {
            audio_driver,
            input_stream: None,
            output_stream: None,
            measurements: Vec::new(),
        }
    }

    /// Add a Measurement to the vector of measurements to be performed
    ///
    /// * `measurement`: The measurement to be added to the suite
    pub fn request_measurement(&mut self, measurement: Box<dyn Measurement>) {
        self.measurements.push(measurement);
    }

    /// Sets the input stream to be used for measurements
    ///
    /// * `stream`: The input stream to be used
    pub fn set_input_stream(&mut self, stream: &StreamChoice) {
        match stream {
            StreamChoice::DefaultDevice() => {
                self.input_stream = Some(self.audio_driver.get_default_source());
            }
            _ => eprint!("Not implemented yet!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::Cell;

    struct MockAudioDriver {
        called_get_default_source: Cell<u8>,
    }

    impl MockAudioDriver {
        fn new() -> Self {
            Self {
                called_get_default_source: Cell::new(0),
            }
        }
    }

    impl AudioDriver for MockAudioDriver {
        fn get_default_source(&self) -> StreamSource {
            let curr = self.called_get_default_source.take();
            self.called_get_default_source.set(curr + 1);
            StreamSource {}
        }
    }

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

    #[test]
    fn setting_streams() {
        let mock_audio_driver = MockAudioDriver::new();
        let mut my_suite = MeasurementSuite::with_audio_driver(mock_audio_driver);

        assert!(my_suite.input_stream.is_none());

        my_suite.set_input_stream(&StreamChoice::DefaultDevice());
        assert!(my_suite.input_stream.is_some());
        assert_eq!(my_suite.audio_driver.called_get_default_source.get(), 1);
    }
}
