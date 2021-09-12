use std::{thread, time::Duration};

use gpio_cdev::{Chip, LineRequestFlags};

const PROGRAM_NAME: &str = "garmon";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", PROGRAM_NAME);

    let mut chip = Chip::new("/dev/gpiochip0")?;
    let output = chip.get_line(16)?;
    let output_handle = output.request(LineRequestFlags::OUTPUT, 0, PROGRAM_NAME)?;

    for i in 1..=10 {
        println!("Blink {}", i);
        output_handle.set_value(1)?;
        thread::sleep(Duration::from_millis(500));
        output_handle.set_value(0)?;
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
