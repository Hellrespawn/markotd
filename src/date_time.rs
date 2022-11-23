use chrono::{Local, NaiveDateTime};
use itertools::Itertools;

pub(crate) struct DateTime;

impl DateTime {
    pub(crate) fn now() -> NaiveDateTime {
        Local::now().naive_local()
    }

    pub(crate) fn format_date(date_time: NaiveDateTime) -> String {
        date_time.format("%c").to_string()
    }

    pub(crate) fn format_duration(duration: u64) -> String {
        let seconds = duration % 60;
        let minutes = (duration / 60) % 60;
        let hours = (duration / 60 / 60) % 60;
        let days = duration / 60 / 60 / 24;

        let times = [seconds, minutes, hours, days];
        let units = ["seconds", "minutes", "hours", "days"];

        let mut strings = times
            .into_iter()
            .zip(units)
            .rev()
            .filter(|(t, _)| *t != 0)
            .map(|(t, u)| format!("{} {}", t, u))
            .intersperse(", ".to_owned())
            .collect::<Vec<_>>();

        let length = strings.len();

        if length > 1 {
            strings[length - 2] = " and ".to_owned();
        }

        strings.join("")
    }
}
