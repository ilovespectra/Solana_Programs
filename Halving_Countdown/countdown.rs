use std::{thread, time::Duration};

const HALVING_INTERVAL: u64 = 52_592_000; // seconds
const HALVING_BLOCKS: u64 = 21_600_000; // blocks

fn main() {
    let mut blocks_left = HALVING_BLOCKS;

    loop {
        let seconds_left = (blocks_left as f64 / 10.0).ceil() as u64;
        let days_left = seconds_left / 86400;
        let hours_left = (seconds_left % 86400) / 3600;
        let minutes_left = (seconds_left % 3600) / 60;
        let seconds_left = seconds_left % 60;

        println!(
            "{} days, {} hours, {} minutes, and {} seconds until the next halving",
            days_left, hours_left, minutes_left, seconds_left
        );

        thread::sleep(Duration::from_secs(HALVING_INTERVAL));
        blocks_left -= HALVING_BLOCKS;
    }
}
