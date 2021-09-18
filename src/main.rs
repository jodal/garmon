use std::{thread, time::Duration};

use garmon::components::{hcsr04::HcSr04, led::Led};
use gpio_cdev::Chip;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", garmon::PROGRAM_NAME);

    let mut chip = Chip::new("/dev/gpiochip0")?;
    let led = Led::new(chip.get_line(16)?)?;
    let hc_sr04 = HcSr04::new(chip.get_line(5)?, chip.get_line(6)?)?;

    loop {
        let distance = hc_sr04.measure_distance_in_cm()?;
        if distance < 20.0 {
            led.on()?;
        } else {
            led.off()?;
        }
        thread::sleep(Duration::from_millis(100));
    }
}
