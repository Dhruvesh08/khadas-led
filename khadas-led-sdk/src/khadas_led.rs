use std::fs::File;
use std::io::{Result, Write};

pub struct LedController {
    trigger_path: String,
}

impl LedController {
    pub fn new(trigger_path: &str) -> Self {
        LedController {
            trigger_path: String::from(trigger_path),
        }
    }

    pub fn set_mode(&self, mode: &str) -> Result<()> {
        let mut file = File::create(&self.trigger_path)?;
        file.write_all(mode.as_bytes())?;
        Ok(())
    }
}


