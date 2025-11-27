use crate::measurement_kit::Measurement;

// mod cli;
mod measurement_kit;
// use crate::measurement_kit;


fn main() {
    let my_measurement = measurement_kit::S21::new("first measurement");

    // Ignore errors
    let _ = my_measurement.run();
}

