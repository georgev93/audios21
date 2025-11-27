// pub use crate::measurement_kit::s21;

pub mod s21;
pub use s21::S21;

/// Types of Measurements available in this kit
pub enum MeasurementTypes {
    /// Perform an S21 insertion loss measurement (with phase information)
    S21(S21)
}

/// Measurement interface
pub trait Measurement {
    type Data;

    fn name(&self) -> &str;
    fn run(&self) -> TestResult;
    fn set_expectation(&mut self, data: Self::Data);
    fn get_data(&self) -> Option<&Self::Data>;
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

