use std::{thread, time::Duration};

use gpio_cdev::{EventRequestFlags, EventType, Line, LineRequestFlags};

pub struct HcSr04 {
    trigger_line: Line,
    echo_line: Line,
}

impl HcSr04 {
    pub fn new(trigger_line: Line, echo_line: Line) -> Result<HcSr04, gpio_cdev::Error> {
        Ok(HcSr04 {
            trigger_line: trigger_line,
            echo_line: echo_line,
        })
    }

    pub fn measure_distance_in_cm(&self) -> Result<f64, gpio_cdev::Error> {
        // Trigger sonic burst
        let trigger =
            self.trigger_line
                .request(LineRequestFlags::OUTPUT, 0, crate::PROGRAM_NAME)?;
        trigger.set_value(1)?;
        thread::sleep(Duration::from_micros(10));
        trigger.set_value(0)?;

        // Measure duration of echo
        let echo_events = self.echo_line.events(
            LineRequestFlags::INPUT,
            EventRequestFlags::BOTH_EDGES,
            crate::PROGRAM_NAME,
        )?;
        let mut rising_edge_timestamp: Option<u64> = None;
        for echo_event in echo_events {
            let evt = echo_event?;
            match evt.event_type() {
                EventType::RisingEdge => {
                    rising_edge_timestamp = Some(evt.timestamp());
                }
                EventType::FallingEdge => match rising_edge_timestamp {
                    Some(timestamp) => {
                        let duration = Duration::from_nanos(evt.timestamp() - timestamp);
                        let speed_of_sound_in_cm_per_micros = 34_300.0 / 1_000_000.0;
                        return Ok(
                            duration.as_micros() as f64 * speed_of_sound_in_cm_per_micros / 2.0,
                        );
                    }
                    None => {}
                },
            }
        }
        unreachable!();
    }
}
