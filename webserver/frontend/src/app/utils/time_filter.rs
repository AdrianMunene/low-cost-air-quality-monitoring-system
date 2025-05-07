use chrono::{DateTime, Utc, Duration};

#[derive(Clone, PartialEq, Debug)]
pub enum TimeRange {
    Today,
    Yesterday,
    LastWeek,
    LastMonth,
    Custom(DateTime<Utc>, DateTime<Utc>), // Start and end dates
}

impl TimeRange {
    // Convert TimeRange to actual date range (start_date, end_date)
    pub fn to_date_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let now = Utc::now();
        log::info!("Current time: {}", now);
        let end = now;

        let start = match self {
            TimeRange::Today => {
                // Start of today
                let start_of_day = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).unwrap();
                log::info!("Today range: {} to {}", start_of_day, now);
                start_of_day
            },
            TimeRange::Yesterday => {
                // Start of yesterday
                let yesterday_start = (now - Duration::days(1)).date_naive().and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).unwrap();
                log::info!("Yesterday range start: {}", yesterday_start);
                yesterday_start
            },
            TimeRange::LastWeek => {
                // 7 days ago
                let week_ago = now - Duration::days(7);
                log::info!("Last week range: {} to {}", week_ago, now);
                week_ago
            },
            TimeRange::LastMonth => {
                // 30 days ago
                let month_ago = now - Duration::days(30);
                log::info!("Last month range: {} to {}", month_ago, now);
                month_ago
            },
            TimeRange::Custom(start_date, end_date) => {
                // Custom date range
                log::info!("Custom range: {} to {}", start_date, end_date);
                *start_date
            },
        };

        // For Yesterday, we need to set the end date to the end of yesterday
        let end = match self {
            TimeRange::Yesterday => {
                let yesterday_end = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).unwrap() - Duration::seconds(1);
                log::info!("Yesterday range end: {}", yesterday_end);
                yesterday_end
            },
            TimeRange::Custom(_, end_date) => {
                *end_date
            },
            _ => end,
        };

        log::info!("Final date range: {} to {}", start, end);
        (start, end)
    }

    // Format for display
    pub fn display_name(&self) -> String {
        match self {
            TimeRange::Today => "Today".to_string(),
            TimeRange::Yesterday => "Yesterday".to_string(),
            TimeRange::LastWeek => "Last 7 Days".to_string(),
            TimeRange::LastMonth => "Last 30 Days".to_string(),
            TimeRange::Custom(start, end) => {
                format!("{} to {}",
                    start.format("%Y-%m-%d"),
                    end.format("%Y-%m-%d"))
            }
        }
    }
}

// Helper function to filter data by time range
pub fn filter_data_by_time_range<T, F>(
    data: &[T],
    time_range: &TimeRange,
    timestamp_extractor: F
) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> Option<DateTime<Utc>>
{
    let (start_date, end_date) = time_range.to_date_range();

    data.iter()
        .filter_map(|item| {
            if let Some(timestamp) = timestamp_extractor(item) {
                if timestamp >= start_date && timestamp <= end_date {
                    Some(item.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}
