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
                
                // Dark grey background
                root_area.fill(&RGBColor(40, 40, 40)).unwrap();

                let mut chart = ChartBuilder::on(&root_area)
                    .set_label_area_size(LabelAreaPosition::Left, pixel_width / 8)
                    .set_label_area_size(LabelAreaPosition::Bottom, pixel_height / 8)
                    .set_label_area_size(LabelAreaPosition::Right, pixel_width / 20)
                    .set_label_area_size(LabelAreaPosition::Top, pixel_height / 20)
                    .caption(
                        &config.caption,
                        ("sans-serif", (pixel_height as f32 * 0.04) as u32).into_font().color(&WHITE)
                    )
                    .build_cartesian_2d(config.x_range.clone(), config.y_range.clone())
                    .unwrap();

                // Configure mesh with white grid and text
                chart.configure_mesh()
                    .x_desc(&config.x_desc)
                    .y_desc(&config.y_desc)
                    .x_labels(config.x_labels)
                    .axis_style(WHITE.mix(0.8))
                    .light_line_style(WHITE.mix(0.2))
                    .bold_line_style(WHITE.mix(0.4))
                    .label_style(("sans-serif", (pixel_height as f32 * 0.025) as u32).into_font().color(&WHITE))
                    .x_label_formatter(&|dt: &DateTime<Utc>| dt.format("%H:%M").to_string())
                    .draw()
                    .unwrap();

                // Draw each series with thicker lines
                for series in config.series.iter() {
                    let style = ShapeStyle::from(&series.color)
                        .stroke_width((pixel_width as f32 * 0.003) as u32);
                    chart.draw_series(
                        LineSeries::new(
                            series.data.iter().map(|p| (p.timestamp, p.value)),
                            style,
                        )
                    ).unwrap()
                    .label(&series.label)
                    .legend(move |(x, y)| {
                        PathElement::new(vec![(x, y), (x + 20, y)], &series.color)
                    });
                }

                // Draw legend with white text
                chart.configure_series_labels()
                    .background_style(&RGBColor(40, 40, 40).mix(0.9))
                    .border_style(&WHITE.mix(0.8))
                    .label_font(("sans-serif", (pixel_height as f32 * 0.025) as u32).into_font().color(&WHITE))
                    .position(SeriesLabelPosition::UpperRight)
                    .margin(10)
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
