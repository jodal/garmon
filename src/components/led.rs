use gpio_cdev::{Line, LineHandle, LineRequestFlags};

pub struct Led {
    handle: LineHandle,
}

impl Led {
    pub fn new(line: Line) -> Result<Led, gpio_cdev::Error> {
        Ok(Led {
            handle: line.request(LineRequestFlags::OUTPUT, 0, crate::PROGRAM_NAME)?,
        })
    }

    pub fn on(&self) -> Result<(), gpio_cdev::Error> {
        self.handle.set_value(1)?;
        Ok(())
    }

    pub fn off(&self) -> Result<(), gpio_cdev::Error> {
        self.handle.set_value(0)?;
        Ok(())
    }
}
