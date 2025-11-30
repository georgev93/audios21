
#[derive(Debug)]
pub struct StreamSource {}

#[derive(Debug)]
pub struct StreamSink {}

pub trait AudioDriver {
    fn get_default_source(&self) -> StreamSource;
}

pub struct AudioDriverImpl {}

impl AudioDriver for AudioDriverImpl {
    fn get_default_source(&self) -> StreamSource {
        StreamSource {}
    }
}

impl AudioDriverImpl {
    #[must_use]
    pub fn new() -> Self {
        AudioDriverImpl {  }
    }
}
