
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

#[cfg(test)]
pub mod mocks {
    use super::*;

    use std::cell::Cell;

    pub struct MockAudioDriver {
        pub called_get_default_source: Cell<u8>,
    }

    impl MockAudioDriver {
        pub fn new() -> Self {
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
}
