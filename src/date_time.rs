use chrono::{Local, NaiveDateTime};
use color_eyre::Result;
use color_eyre::eyre::eyre;
use itertools::Itertools;

use crate::Config;
use crate::model::DateTime;

const DIVISORS: [u64; 4] = [60, 60, 24, 7];
const UNITS: [&str; 5] = ["s", "m", "h", "d", "w"];

pub const MAX_DIVISIONS: usize = DIVISORS.len();

impl DateTime {
    pub(crate) fn now() -> NaiveDateTime {
        Local::now().naive_local()
    }

    pub(crate) fn format_date(date_time: NaiveDateTime) -> String {
        date_time.format("%c").to_string()
    }

    pub(crate) fn format_duration(
        seconds: u64,
        config: &Config,
    ) -> Result<String> {
        let num_divisions = config.duration_divisions();

        debug_assert_eq!(
            DIVISORS.len(),
            UNITS.len() - 1,
            "`units.len()` must be `divisors.len() + 1`"
        );

        if num_divisions > MAX_DIVISIONS {
            return Err(eyre!(
                "`divisions` may not be bigger than {}.",
                MAX_DIVISIONS
            ));
        }

        let handle_division =
            |i| Self::handle_division(i, num_divisions, &DIVISORS, seconds);

        #[allow(clippy::needless_collect)]
        let times =
            (0..=num_divisions).map(handle_division).collect::<Vec<_>>();

        let formatted = times
            .into_iter()
            .zip(UNITS)
            .rev()
            .filter(|(t, _)| *t != 0)
            .map(|(t, u)| format!("{}{}", t, u))
            .intersperse(" ".to_owned())
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

            if i != divisions
                && let Some(modu) = divisors.get(i)
            {
                time %= modu;
            }
        }

        time
    }
}
