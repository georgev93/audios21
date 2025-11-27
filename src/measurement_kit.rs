// pub use crate::measurement_kit::s21;

pub mod s21;
pub use s21::S21;

/// Fields shared by all measurement types
pub struct MeasurementCommon<T> {
    /// Name of the measurement
    name: String,

    /// Whether or not the measurement has been taken
    ran: bool,

    /// The data collected by the measurement
    data: Option<T>,

    /// The expected result (if this test is pass/fail)
    expected_result: Option<T>,
}

pub trait MeasurementData {
    fn interpolate(&mut self);
    fn get_point(&self, dependent_var: f64) -> f64;
}

/// Possible outcomes of a test that ran with no errors
pub enum TestOutcome {
    /// When no pass/fail requirements were checked and the test successfully completed
    Ran,

    /// When the test successfully ran and passed the predefined pass/fail requirement
    Passed,

    /// When the test successfully ran and failed the predefined pass/fail requirement
    Failed,
}

/// Either a `TestOutcome` or an error thrown while running
pub type TestResult = Result<TestOutcome, std::io::Error>;

pub trait Measurement {
    type MeasurementData;

    fn name(&self) -> &str;
    fn run(&self) -> TestResult;
    fn set_expectation(&mut self, data: Self::MeasurementData);
    fn get_data(&self) -> Option<&Self::MeasurementData>;
}

pub enum MeasurementTypes {
    S21(s21::S21)
}
