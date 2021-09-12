use std::{thread, time::Duration};

use gpio_cdev::{Chip, EventRequestFlags, EventType, Line, LineRequestFlags};

const PROGRAM_NAME: &str = "garmon";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", PROGRAM_NAME);

    let mut chip = Chip::new("/dev/gpiochip0")?;
    let led_line = chip.get_line(16)?;
    let led_handle = led_line.request(LineRequestFlags::OUTPUT, 0, PROGRAM_NAME)?;
    let hc_sr04_trigger_line = chip.get_line(5)?;
    let hc_sr04_echo_line = chip.get_line(6)?;

    loop {
        let distance = read_distance_in_cm(&hc_sr04_trigger_line, &hc_sr04_echo_line)?;
        if distance < 20.0 {
            led_handle.set_value(1)?;
        } else {
            led_handle.set_value(0)?;
        }
        thread::sleep(Duration::from_millis(100));
    }
}

#[allow(dead_code)]
fn read_distance_in_cm(trigger_line: &Line, echo_line: &Line) -> Result<f64, gpio_cdev::Error> {
    let trigger_handle = trigger_line.request(LineRequestFlags::OUTPUT, 0, PROGRAM_NAME)?;

    // Trigger sonic burst
    trigger_handle.set_value(1)?;
    thread::sleep(Duration::from_micros(10));
    trigger_handle.set_value(0)?;

    // Measure duration of echo
    let mut rising_edge_timestamp: Option<u64> = None;
    for echo_event in echo_line.events(
        LineRequestFlags::INPUT,
        EventRequestFlags::BOTH_EDGES,
        PROGRAM_NAME,
    )? {
        let evt = echo_event?;
        match evt.event_type() {
            EventType::RisingEdge => {
                rising_edge_timestamp = Some(evt.timestamp());
            }
            EventType::FallingEdge => match rising_edge_timestamp {
                Some(timestamp) => {
                    let duration = Duration::from_nanos(evt.timestamp() - timestamp);
                    let speed_of_sound_in_cm_per_micros = 34_300.0 / 1_000_000.0;
                    return Ok(duration.as_micros() as f64 * speed_of_sound_in_cm_per_micros / 2.0);
                }
                None => {}
            },
        }
    }
    unreachable!();
}

#[allow(dead_code)]
fn blink_led(led_line: &Line) -> Result<(), gpio_cdev::Error> {
    let led_handle = led_line.request(LineRequestFlags::OUTPUT, 0, PROGRAM_NAME)?;
    for i in 1..=10 {
        println!("Blink {}", i);
        led_handle.set_value(1)?;
        thread::sleep(Duration::from_millis(500));
        led_handle.set_value(0)?;
        thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}
