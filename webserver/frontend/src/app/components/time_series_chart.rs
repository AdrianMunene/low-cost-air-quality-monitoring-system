use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use plotters_canvas::CanvasBackend;
use web_sys::{HtmlCanvasElement, window};
use plotters::prelude::*;
use chrono::{DateTime, Utc, Timelike, Datelike};
use std::ops::Range;

/// A single data point.
#[derive(Clone, PartialEq)]
pub struct DataPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
}

/// A data series (line) to be drawn on the chart.
#[derive(Clone, PartialEq)]
pub struct ChartSeries {
    pub label: String,
    pub data: Vec<DataPoint>,
    pub color: RGBColor,
}

/// Overall configuration for the time-series chart.
#[derive(Clone, PartialEq)]
pub struct TimeSeriesChartConfig {
    pub caption: String,
    pub x_desc: String,
    pub y_desc: String,
    pub x_labels: usize,
    pub x_range: Range<DateTime<Utc>>,
    pub y_range: Range<f64>,
    pub series: Vec<ChartSeries>,
}

/// Props for the TimeSeriesChart component.
#[derive(Properties, PartialEq)]
pub struct TimeSeriesChartProps {
    pub config: TimeSeriesChartConfig,
}

#[function_component(TimeSeriesChart)]
pub fn time_series_chart(props: &TimeSeriesChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let error_state = use_state(|| false);

    {
        let config = props.config.clone();
        let canvas_ref = canvas_ref.clone();
        let error_state = error_state.clone();

        use_effect_with((), move |_| {
            let canvas = canvas_ref.cast::<HtmlCanvasElement>();
            if canvas.is_none() {
                error_state.set(true);
                return ();
            }

            let canvas = canvas.unwrap();

            // Create a render function
            let render_closure = Closure::wrap(Box::new(move || {
                // Check if canvas has size
                let w = canvas.client_width();
                let h = canvas.client_height();

                if w <= 0 || h <= 0 {
                    error_state.set(true);
                    return;
                }

                // 1) Handle DPI with higher resolution
                let dpr = window().unwrap().device_pixel_ratio();
                // Use a higher scaling factor for better quality
                let scaling_factor = dpr * 1.5; // Increase resolution by 50%
                let w = w as u32;
                let h = h as u32;
                canvas.set_width((w as f64 * scaling_factor) as u32);
                canvas.set_height((h as f64 * scaling_factor) as u32);

                // 2) Setup Plotters backend
                let backend = CanvasBackend::with_canvas_object(canvas.clone())
                    .expect("Failed to create canvas backend");

                let root = backend.into_drawing_area();
                // Use an even darker background color similar to the reference chart
                root.fill(&RGBColor(15, 15, 25)).unwrap();

                // Reset thread_local variables for x-axis labels
                thread_local! {
                    static PREV_HOUR: std::cell::RefCell<Option<i32>> = std::cell::RefCell::new(None);
                    static PREV_DAY: std::cell::RefCell<Option<i32>> = std::cell::RefCell::new(None);
                    static PREV_MONTH: std::cell::RefCell<Option<i32>> = std::cell::RefCell::new(None);
                    static PREV_YEAR: std::cell::RefCell<Option<i32>> = std::cell::RefCell::new(None);
                }

                PREV_HOUR.with(|h| *h.borrow_mut() = None);
                PREV_DAY.with(|d| *d.borrow_mut() = None);
                PREV_MONTH.with(|m| *m.borrow_mut() = None);
                PREV_YEAR.with(|y| *y.borrow_mut() = None);

                // 3) Build chart with better margins and label areas
                let mut chart = ChartBuilder::on(&root)
                    .set_label_area_size(LabelAreaPosition::Left, w / 6) // Even larger left margin for y-axis labels
                    .set_label_area_size(LabelAreaPosition::Bottom, h / 6) // Larger bottom for x-axis labels
                    .set_label_area_size(LabelAreaPosition::Right, 0) // No right margin/labels
                    .set_label_area_size(LabelAreaPosition::Top, 0)   // No top margin/labels
                    .margin(20) // Larger margin for better spacing
                    .caption(&config.caption, ("sans-serif", (h as f32 * 0.06) as u32).into_font().color(&WHITE))
                    .build_cartesian_2d(
                        config.x_range.clone(),
                        config.y_range.clone(),
                    )
                    .unwrap();

                chart
                    .configure_mesh()
                    // Use a grid similar to the reference chart
                    .x_labels(10) // Balanced number of x-axis labels for our smart formatting
                    .y_labels(8) // More y-axis labels
                    .axis_style(WHITE.mix(0.9)) // More visible axes
                    .light_line_style(WHITE.mix(0.15)) // Slightly more visible grid lines
                    .bold_line_style(WHITE.mix(0.25)) // Slightly more visible bold lines
                    .label_style(
                        ("sans-serif", (h as f32 * 0.05) as u32) // Slightly smaller font to avoid clipping
                            .into_font()
                            .color(&WHITE), // Pure white for maximum contrast
                    )
                    .axis_desc_style(("sans-serif", (h as f32 * 0.06) as u32).into_font().color(&WHITE))
                    .x_desc(&config.x_desc)
                    .y_desc(&config.y_desc)
                    .x_label_formatter(&|dt: &DateTime<Utc>| {
                        // Smart time formatting logic
                        // Using the thread_local variables defined at chart level

                        let hour = dt.hour() as i32;
                        let day = dt.day() as i32;
                        let month = dt.month() as i32;
                        let year = dt.year();
                        let minute = dt.minute();

                        // Format based on changes
                        let result = PREV_YEAR.with(|prev_year| {
                            PREV_MONTH.with(|prev_month| {
                                PREV_DAY.with(|prev_day| {
                                    PREV_HOUR.with(|prev_hour| {
                                        let mut prev_hour_val = prev_hour.borrow_mut();
                                        let mut prev_day_val = prev_day.borrow_mut();
                                        let mut prev_month_val = prev_month.borrow_mut();
                                        let mut prev_year_val = prev_year.borrow_mut();

                                        let result = if prev_year_val.is_none() || *prev_year_val != Some(year) {
                                            // Year changed or first label
                                            *prev_year_val = Some(year);
                                            *prev_month_val = Some(month);
                                            *prev_day_val = Some(day);
                                            *prev_hour_val = Some(hour);
                                            format!("{}/{}/{} {:02}:{:02}", year, month, day, hour, minute)
                                        } else if prev_month_val.is_none() || *prev_month_val != Some(month) {
                                            // Month changed
                                            *prev_month_val = Some(month);
                                            *prev_day_val = Some(day);
                                            *prev_hour_val = Some(hour);
                                            format!("{}/{} {:02}:{:02}", month, day, hour, minute)
                                        } else if prev_day_val.is_none() || *prev_day_val != Some(day) {
                                            // Day changed
                                            *prev_day_val = Some(day);
                                            *prev_hour_val = Some(hour);
                                            format!("{} {:02}:{:02}", day, hour, minute)
                                        } else if prev_hour_val.is_none() || *prev_hour_val != Some(hour) {
                                            // Hour changed
                                            *prev_hour_val = Some(hour);
                                            format!("{:02}:{:02}", hour, minute)
                                        } else {
                                            // Only minute changed
                                            format!("{:02}", minute)
                                        };

                                        result
                                    })
                                })
                            })
                        });

                        result
                    })
                    .y_label_formatter(&|y| format!("{:.1}", y))
                    .draw()
                    .unwrap();

                // 4) Plot series
                for series in &config.series {
                    // Skip empty series
                    if series.data.is_empty() {
                        continue;
                    }

                    let color = match series.label.as_str() {
                        // Use colors similar to the reference chart
                        "PM 1.0" => RGBColor(59, 130, 246),  // Blue
                        "PM 2.5" => RGBColor(255, 165, 0),   // Orange like in reference
                        "PM 10"  => RGBColor(239, 68, 68),   // Red
                        "Temperature" => RGBColor(244, 63, 94), // Red
                        "Humidity"    => RGBColor(16, 185, 129), // Green
                        "Pressure"    => RGBColor(139, 92, 246), // Purple
                        "CO₂"         => RGBColor(255, 165, 0),  // Orange like in reference
                        "CO"          => RGBColor(244, 63, 94),  // Red
                        "O₃"          => RGBColor(59, 130, 246), // Blue
                        _ => series.color.clone(),
                    };

                    let style = ShapeStyle::from(&color)
                        .stroke_width(5); // Even thicker lines for better visibility

                    // Draw the line series
                    chart
                        .draw_series(LineSeries::new(
                            series.data.iter().map(|p| (p.timestamp, p.value)),
                            style.clone(),
                        ))
                        .unwrap()
                        .label(&series.label)
                        .legend(move |(x, y)| {
                            PathElement::new(vec![(x, y), (x + 20, y)], &color)
                        });

                    // Determine if we should show points based on data density
                    // Calculate the time span of the data
                    if !series.data.is_empty() {
                        let time_span = if series.data.len() > 1 {
                            let first = series.data.first().unwrap().timestamp;
                            let last = series.data.last().unwrap().timestamp;
                            (last - first).num_seconds() as f64
                        } else {
                            // Default span if only one point
                            86400.0 // One day in seconds
                        };

                        // Calculate average time between points
                        let avg_time_between_points = time_span / series.data.len() as f64;

                        // Show points if the average time between points is more than 2 hours
                        // or if there are fewer than 30 points total
                        if avg_time_between_points > 7200.0 || series.data.len() < 30 {
                            chart
                                .draw_series(series.data.iter().map(|pt| {
                                    Circle::new(
                                        (pt.timestamp, pt.value),
                                        6, // Larger fixed-size points for better visibility
                                        style.filled(),
                                    )
                                }))
                                .unwrap();
                        }
                    }
                }

                // 5) Legend - similar to reference chart
                chart
                    .configure_series_labels()
                    .background_style(&RGBColor(30, 30, 40).mix(0.9)) // Slightly transparent background
                    .border_style(&WHITE.mix(0.3)) // Subtle border
                    .label_font(
                        ("sans-serif", (h as f32 * 0.055) as u32) // Larger font for legend
                            .into_font()
                            .color(&WHITE), // Pure white for maximum contrast
                    )
                    .position(SeriesLabelPosition::UpperLeft) // Position in upper left to avoid right side
                    .margin(15) // Larger margin
                    .draw()
                    .unwrap();
            }) as Box<dyn FnMut()>);

            // Request animation frame
            window()
                .unwrap()
                .request_animation_frame(render_closure.as_ref().unchecked_ref())
                .unwrap();

            // Prevent closure from being garbage collected
            render_closure.forget();

            ()
        });
    }

    html! {
        <div class="chart-container-inner">
            <canvas
                ref={canvas_ref}
                style="position:absolute;top:0;left:0;width:100%;height:100%;display:block;image-rendering:high-quality;-webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:grayscale;"
            />
            { if *error_state {
                html! { <div class="chart-error">{ "Unable to render chart. Please check your data." }</div> }
            } else {
                html! {}
            }}
        </div>
    }
}
