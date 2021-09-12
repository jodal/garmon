use std::{thread, time::Duration};

use gpio_cdev::{Chip, EventRequestFlags, EventType, LineRequestFlags};

const PROGRAM_NAME: &str = "garmon";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", PROGRAM_NAME);

    loop {
        match read_distance_in_cm()? {
            Some(distance) => {
                println!("Distance: {:3.1} cm", &distance);
            }
            None => {
                println!("Distance: read failed");
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
}

fn read_distance_in_cm() -> Result<Option<f64>, gpio_cdev::Error> {
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let trigger = chip.get_line(5)?;
    let trigger_handle = trigger.request(LineRequestFlags::OUTPUT, 0, PROGRAM_NAME)?;
    let echo = chip.get_line(6)?;

    // Trigger sonic burst
    trigger_handle.set_value(1)?;
    thread::sleep(Duration::from_micros(10));
    trigger_handle.set_value(0)?;

    // Measure duration of echo
    let mut rising_edge_timestamp: Option<u64> = None;
    for event in echo.events(
        LineRequestFlags::INPUT,
        EventRequestFlags::BOTH_EDGES,
        PROGRAM_NAME,
    )? {
        let evt = event?;
        match evt.event_type() {
            EventType::RisingEdge => {
                rising_edge_timestamp = Some(evt.timestamp());
            }
            EventType::FallingEdge => match rising_edge_timestamp {
                Some(timestamp) => {
                    let duration = Duration::from_nanos(evt.timestamp() - timestamp);
                    let speed_of_sound_in_cm_per_micros = 34_300.0 / 1_000_000.0;
                    return Ok(Some(
                        duration.as_micros() as f64 * speed_of_sound_in_cm_per_micros / 2.0,
                    ));
                }
                None => {}
            },
        }
    }

    Ok(None)
}

#[allow(dead_code)]
fn blink_led() -> Result<(), gpio_cdev::Error> {
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
