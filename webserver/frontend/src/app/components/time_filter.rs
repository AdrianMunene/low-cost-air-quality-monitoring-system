use yew::prelude::*;
use chrono::{DateTime, Utc};
use crate::app::utils::time_filter::TimeRange;
use crate::app::utils::parse_timestamp::parse_timestamp;
use web_sys::{HtmlSelectElement, HtmlInputElement};

#[derive(Properties, Clone, PartialEq)]
pub struct TimeFilterProps {
    pub selected_range: TimeRange,
    pub on_range_change: Callback<TimeRange>,
}

#[function_component(TimeFilterComponent)]
pub fn time_filter(props: &TimeFilterProps) -> Html {
    // State for showing custom date inputs - initialize based on current selection
    let show_custom_dates = use_state(|| {
        matches!(props.selected_range, TimeRange::Custom(_, _))
    });

    // State for custom date values
    let start_date = use_state(|| String::from(""));
    let end_date = use_state(|| String::from(""));

    // Initialize custom date values if we're already in custom mode
    {
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        let show_custom_dates = show_custom_dates.clone();

        use_effect_with(props.selected_range.clone(), move |selected_range| {
            if let TimeRange::Custom(start, end) = selected_range {
                // Format dates for input fields (YYYY-MM-DD)
                start_date.set(start.format("%Y-%m-%d").to_string());
                end_date.set(end.format("%Y-%m-%d").to_string());
                show_custom_dates.set(true);
            } else {
                // For non-custom ranges, initialize with reasonable defaults
                let now = chrono::Utc::now();
                let one_week_ago = now - chrono::Duration::days(7);

                // Set default date range (last 7 days)
                start_date.set(one_week_ago.format("%Y-%m-%d").to_string());
                end_date.set(now.format("%Y-%m-%d").to_string());
            }
            || ()
        });
    }

    let on_select = {
        let on_range_change = props.on_range_change.clone();
        let show_custom_dates = show_custom_dates.clone();

        Callback::from(move |e: Event| {
            let select = e.target_dyn_into::<HtmlSelectElement>();
            if let Some(select) = select {
                let value = select.value();

                match value.as_str() {
                    "custom" => {
                        // Show custom date inputs
                        show_custom_dates.set(true);
                        // Don't emit a change yet, wait for date selection
                    },
                    _ => {
                        // Hide custom date inputs
                        show_custom_dates.set(false);

                        // Create the appropriate TimeRange
                        let range = match value.as_str() {
                            "today" => TimeRange::Today,
                            "yesterday" => TimeRange::Yesterday,
                            "last_week" => TimeRange::LastWeek,
                            "last_month" => TimeRange::LastMonth,
                            _ => return,
                        };

                        // Emit the change
                        on_range_change.emit(range);
                    }
                }
            }
        })
    };

    let on_start_date_change = {
        let start_date = start_date.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                start_date.set(input.value());
            }
        })
    };

    let on_end_date_change = {
        let end_date = end_date.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                end_date.set(input.value());
            }
        })
    };

    let on_apply_custom_range = {
        let on_range_change = props.on_range_change.clone();
        let start_date = start_date.clone();
        let end_date = end_date.clone();

        Callback::from(move |_: MouseEvent| {
            log::info!("Applying custom date range: start={}, end={}", *start_date, *end_date);

            // Parse the dates
            if let (Some(start), Some(end)) = (parse_date(&start_date), parse_date(&end_date)) {
                log::info!("Successfully parsed dates: start={}, end={}", start, end);

                // Create a custom TimeRange
                let range = TimeRange::Custom(start, end);

                // Emit the change
                on_range_change.emit(range);
            } else {
                log::error!("Failed to parse custom date range");
            }
        })
    };

    // Determine which option should be selected
    let selected_value = match &props.selected_range {
        TimeRange::Today => {
            log::info!("Selected range is Today");
            "today"
        },
        TimeRange::Yesterday => {
            log::info!("Selected range is Yesterday");
            "yesterday"
        },
        TimeRange::LastWeek => {
            log::info!("Selected range is Last Week");
            "last_week"
        },
        TimeRange::LastMonth => {
            log::info!("Selected range is Last Month");
            "last_month"
        },
        TimeRange::Custom(_, _) => {
            log::info!("Selected range is Custom");
            "custom"
        },
    };

    log::info!("Selected value for dropdown: {}", selected_value);

    html! {
        <div class="time-filter">
            <div class="time-filter-row">
                {
                    if *show_custom_dates {
                        // Show only custom date inputs when in custom mode
                        html! {
                            <>
                                <label for="start-date">{"Start Date:"}</label>
                                <input
                                    type="date"
                                    id="start-date"
                                    value={(*start_date).clone()}
                                    onchange={on_start_date_change}
                                />

                                <label for="end-date">{"End Date:"}</label>
                                <input
                                    type="date"
                                    id="end-date"
                                    value={(*end_date).clone()}
                                    onchange={on_end_date_change}
                                />

                                <div class="button-group">
                                    <button onclick={on_apply_custom_range} class="apply-button">{"✓"}</button>
                                    <button onclick={
                                        let on_range_change = props.on_range_change.clone();
                                        let show_custom_dates = show_custom_dates.clone();
                                        let is_custom = matches!(props.selected_range, TimeRange::Custom(_, _));

                                        Callback::from(move |_| {
                                            // If we're already in custom mode, we need to revert to a standard range
                                            if is_custom {
                                                // Default to Last 7 Days when canceling from custom
                                                on_range_change.emit(TimeRange::LastWeek);
                                            }
                                            show_custom_dates.set(false);
                                        })
                                    } class="cancel-button">{"✕"}</button>
                                </div>
                            </>
                        }
                    } else {
                        // Show dropdown when not in custom mode
                        html! {
                            <>
                                <label for="time-range" class="time-range-label">{"Time Range:"}</label>
                                <select id="time-range" value={selected_value} onchange={on_select} class="time-range-select">
                                    <option value="today">{"Today"}</option>
                                    <option value="yesterday">{"Yesterday"}</option>
                                    <option value="last_week">{"Last 7 Days"}</option>
                                    <option value="last_month">{"Last 30 Days"}</option>
                                    <option value="custom">{"Custom Range"}</option>
                                </select>
                            </>
                        }
                    }
                }
            </div>
        </div>
    }
}

// Helper function to parse a date string (YYYY-MM-DD) to DateTime<Utc>
fn parse_date(date_str: &str) -> Option<DateTime<Utc>> {
    // Format the date string to include time
    let datetime_str = format!("{} 00:00:00 +0000", date_str);

    // Use the parse_timestamp function
    match parse_timestamp(&datetime_str) {
        Ok(dt) => Some(dt),
        Err(e) => {
            log::warn!("Failed to parse date: {} - Error: {}", date_str, e);
            None
        }
    }
}
