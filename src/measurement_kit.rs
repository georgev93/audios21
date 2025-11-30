pub mod s21;

/// Measurement interface
pub trait Measurement {
    fn name(&self) -> &str;
    fn run(&self) -> TestResult;
    fn calibrate(&self) -> Result<(), std::io::Error> ;
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

