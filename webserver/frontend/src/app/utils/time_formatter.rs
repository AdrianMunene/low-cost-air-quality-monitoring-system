use chrono::{DateTime, Utc, Timelike, Datelike};
use std::cell::RefCell;

thread_local! {
    // Keep track of the last‐seen year/month/day/hour
    static PREV: RefCell<TimeLabelState> = RefCell::new(TimeLabelState::new());
}

struct TimeLabelState {
    year:  Option<i32>,
    month: Option<u32>,
    day:   Option<u32>,
    hour:  Option<u32>,
}

impl TimeLabelState {
    fn new() -> Self {
        TimeLabelState { year: None, month: None, day: None, hour: None }
    }
}

/// Reset the thread-local state to ensure the next call to smart_time_label
/// will show the full date format
pub fn reset_time_label_state() {
    PREV.with(|cell| {
        *cell.borrow_mut() = TimeLabelState::new();
    });
}

/// Given a DateTime<Utc>, only show the largest unit that has changed:
///   * On year change, show `YYYY/MM/DD HH:MM`
///   * On month change, show `MM/DD HH:MM`
///   * On day change, show `DD HH:MM`
///   * On hour change, show `HH:MM`
///   * Otherwise, show `:MM`
pub fn smart_time_label(dt: &DateTime<Utc>) -> String {
    PREV.with(|cell| {
        let mut state = cell.borrow_mut();
        let y = dt.year();
        let m = dt.month();
        let d = dt.day();
        let h = dt.hour();
        let min = dt.minute();

        let label = if state.year != Some(y) {
            // new year
            state.year  = Some(y);
            state.month = Some(m);
            state.day   = Some(d);
            state.hour  = Some(h);
            format!("{:04}/{:02}/{:02} {:02}:{:02}", y, m, d, h, min)
        } else if state.month != Some(m) {
            // new month
            state.month = Some(m);
            state.day   = Some(d);
            state.hour  = Some(h);
            format!("{:02}/{:02} {:02}:{:02}", m, d, h, min)
        } else if state.day != Some(d) {
            // new day
            state.day  = Some(d);
            state.hour = Some(h);
            format!("{:02} {:02}:{:02}", d, h, min)
        } else if state.hour != Some(h) {
            // new hour
            state.hour = Some(h);
            format!("{:02}:{:02}", h, min)
        } else {
            // same hour
            format!(":{:02}", min)
        };

        label
    })
}
