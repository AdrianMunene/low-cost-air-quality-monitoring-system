use yew::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use plotters::prelude::*;
use chrono::{DateTime, Utc};
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

    {
        let config = props.config.clone();
        let canvas_ref = canvas_ref.clone();
        use_effect_with(config, move |config| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                // Get device pixel ratio for high DPI displays
                let dpr = web_sys::window()
                    .unwrap()
                    .device_pixel_ratio();

                // Get canvas display size
                let display_width = canvas.client_width() as u32;
                let display_height = canvas.client_height() as u32;

                // Calculate actual pixel dimensions
                let pixel_width = (display_width as f64 * dpr) as u32;
                let pixel_height = (display_height as f64 * dpr) as u32;

                // Set canvas buffer dimensions
                canvas.set_width(pixel_width);
                canvas.set_height(pixel_height);

                let backend = CanvasBackend::with_canvas_object(canvas)
                    .expect("Failed to create CanvasBackend");
                let root_area = backend.into_drawing_area();

                // Darker background for better contrast
                root_area.fill(&RGBColor(20, 20, 20)).unwrap();

                // Create chart with optimized margins for grid layout
                let mut chart = ChartBuilder::on(&root_area)
                    .set_label_area_size(LabelAreaPosition::Left, pixel_width / 10)
                    .set_label_area_size(LabelAreaPosition::Bottom, pixel_height / 12)
                    .set_label_area_size(LabelAreaPosition::Right, 0)
                    .set_label_area_size(LabelAreaPosition::Top, 0)
                    .margin(5)
                    .build_cartesian_2d(config.x_range.clone(), config.y_range.clone())
                    .unwrap();

                // Configure mesh with optimized styling for grid layout
                chart.configure_mesh()
                    .disable_x_mesh() // Disable vertical grid lines for cleaner look
                    .y_labels(4) // Fewer labels for cleaner look
                    .x_labels(4)
                    .axis_style(WHITE.mix(0.7)) // Visible axis lines
                    .light_line_style(WHITE.mix(0.1)) // Very subtle grid lines
                    .bold_line_style(WHITE.mix(0.2)) // Subtle bold lines
                    .label_style(
                        ("sans-serif", (pixel_height as f32 * 0.022) as u32)
                            .into_font()
                            .color(&WHITE.mix(0.8))
                    )
                    .x_label_formatter(&|dt: &DateTime<Utc>| dt.format("%H:%M").to_string())
                    .y_label_formatter(&|y| format!("{:.0}", y)) // No decimal places for cleaner look
                    .draw()
                    .unwrap();

                // Draw each series with improved styling
                for series in config.series.iter() {
                    // Use accent colors for better visibility
                    let color = match series.label.as_str() {
                        "PM 1.0" => RGBColor(59, 130, 246),   // Blue
                        "PM 2.5" => RGBColor(245, 158, 11),   // Amber
                        "PM 10" => RGBColor(239, 68, 68),     // Red
                        "Temperature" => RGBColor(239, 68, 68),  // Red
                        "Humidity" => RGBColor(16, 185, 129),   // Green
                        "Pressure" => RGBColor(124, 58, 237),   // Purple
                        "CO₂" => RGBColor(245, 158, 11),       // Amber
                        "CO" => RGBColor(239, 68, 68),         // Red
                        "O₃" => RGBColor(59, 130, 246),        // Blue
                        _ => series.color,
                    };

                    // Create a thicker line for better visibility
                    let line_style = ShapeStyle::from(&color)
                        .stroke_width((pixel_width as f32 * 0.004) as u32);

                    // Draw the line series
                    chart.draw_series(
                        LineSeries::new(
                            series.data.iter().map(|p| (p.timestamp, p.value)),
                            line_style,
                        )
                    ).unwrap()
                    .label(&series.label)
                    .legend(move |(x, y)| {
                        PathElement::new(vec![(x, y), (x + 20, y)], &color)
                    });

                    // Add data points for better visibility
                    if series.data.len() < 30 {
                        chart.draw_series(
                            series.data.iter().map(|point| {
                                Circle::new(
                                    (point.timestamp, point.value),
                                    (pixel_width as f32 * 0.0025) as u32,
                                    ShapeStyle::from(&color).filled(),
                                )
                            })
                        ).unwrap();
                    }
                }

                // Draw better legend
                chart.configure_series_labels()
                    .background_style(&RGBColor(35, 35, 35).mix(0.9)) // Dark background
                    .border_style(&WHITE.mix(0.3))
                    .label_font(
                        ("sans-serif", (pixel_height as f32 * 0.025) as u32)
                            .into_font()
                            .color(&WHITE.mix(0.95))
                    )
                    .position(SeriesLabelPosition::UpperRight)
                    .margin(5)
                    .draw()
                    .unwrap();
            }
            || {}
        });
    }

    html! {
        <div style="width: 100%; height: 100%; position: relative;">
            <canvas ref={canvas_ref} style="width: 100%; height: 100%;"/>
        </div>
    }
}
