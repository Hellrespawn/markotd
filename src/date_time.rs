use chrono::{Local, NaiveDateTime};
use color_eyre::Result;
use itertools::Itertools;
use serde::Serialize;

use crate::Config;

const DIVISORS: [u64; 4] = [60, 60, 24, 7];
const UNITS: [&str; 5] = ["s", "m", "h", "d", "w"];

pub const MAX_DIVISIONS: usize = DIVISORS.len();

#[derive(Serialize)]
pub struct DateTime {
    time_since: String,
    date: String,
}

impl DateTime {
    pub fn new(time_since: String, date: String) -> Self {
        Self { time_since, date }
    }

    pub(crate) fn now() -> NaiveDateTime {
        Local::now().naive_local()
    }

    pub(crate) fn format_date(date_time: NaiveDateTime) -> String {
        date_time.format("%c").to_string()
    }

    pub(crate) fn format_duration(seconds: u64) -> Result<String> {
        let num_divisions = Config::duration_divisions()?;

        assert_eq!(
            DIVISORS.len(),
            UNITS.len() - 1,
            "`units.len()` must be `divisors.len() + 1`"
        );

        assert!(
            num_divisions <= MAX_DIVISIONS,
            "`divisions` may not be bigger than {}.",
            MAX_DIVISIONS
        );

        // Curry function
        let handle_division =
            |i| Self::handle_division(i, num_divisions, &DIVISORS, seconds);

        #[allow(clippy::needless_collect)]
        let times =
            (0..=num_divisions).map(handle_division).collect::<Vec<_>>();

        let formatted = times
            .into_iter()
            .zip(UNITS)
            .rev() // Longest to shortest unit
            .filter(|(t, _)| *t != 0) // Remove times with value 0
            .map(|(t, u)| format!("{}{}", t, u)) // Format times
            .intersperse(" ".to_owned()) // Join Strings
            .collect();

        Ok(formatted)
    }

    fn handle_division(
        i: usize,
        divisions: usize,
        divisors: &[u64],
        seconds: u64,
    ) -> u64 {
        let current_divisors = divisors.get(0..i);

        let mut time = seconds;

        if let Some(current_divisors) = current_divisors {
            for div in current_divisors {
                time /= div;
            }

            if i != divisions {
                if let Some(modu) = divisors.get(i) {
                    time %= modu;
                }
            }
        }

        time
    }
}
