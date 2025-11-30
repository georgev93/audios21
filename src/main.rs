//! This is my crate doc

use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use audios21::{MeasurementSuite, StreamChoice};

fn main() {
    let mut my_suite = MeasurementSuite::new();
    my_suite.set_input_stream(&StreamChoice::DefaultDevice());
    // let my_measurement = measurement_kit::s21::S21::new("first measurement");

    // Ignore errors
    // let _ = my_measurement.run();

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");

    let mut support_configs_range = device
        .supported_input_configs()
        .expect("error while querying configs");
    let supported_config = support_configs_range
        .next()
        .expect("no supported config!?")
        .with_max_sample_rate();

    let config = supported_config.config();

    dbg!(&config);

    let stream = device
        .build_input_stream(
            &config,
            |_: &[f32], _: &cpal::InputCallbackInfo| println!("listening"),
            |_| println!("Encountered an error"),
            Some(Duration::new(5, 0)),
        )
        .unwrap();

    stream.play().unwrap();

    std::thread::sleep(Duration::new(10, 0));
}
