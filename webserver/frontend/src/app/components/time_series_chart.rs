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
        // use_effect_with accepts a dependency (here, the config) that must implement PartialEq.
        use_effect_with(config, move |config| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let backend = CanvasBackend::with_canvas_object(canvas)
                    .expect("Failed to create CanvasBackend");
                let root_area = backend.into_drawing_area();
                // Clear the drawing area.
                root_area.fill(&RGBColor(20, 20, 30)).unwrap();

                // Build the chart.
                let mut chart = ChartBuilder::on(&root_area)
                    .set_label_area_size(LabelAreaPosition::Left, 100)
                    .set_label_area_size(LabelAreaPosition::Bottom, 100)
                    .caption(&config.caption, ("sans-serif", 40).into_font().color(&WHITE))
                    .build_cartesian_2d(config.x_range.clone(), config.y_range.clone())
                    .unwrap();

                // Configure the mesh.
                chart.configure_mesh()
                    .x_desc(&config.x_desc)
                    .y_desc(&config.y_desc)
                    .x_labels(config.x_labels)
                    .x_label_formatter(&|dt: &DateTime<Utc>| dt.format("%Y-%m-%d").to_string())
                    .draw()
                    .unwrap();

                // Draw each series.
                for series in config.series.iter() {
                    let style = ShapeStyle::from(&series.color).stroke_width(4);
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

                // Draw the legend.
                chart.configure_series_labels()
                    .background_style(&RGBColor(20, 20, 30).mix(0.9))
                    .border_style(&WHITE)
                    .label_font(("sans-serif", 20).into_font().color(&WHITE))
                    .position(SeriesLabelPosition::MiddleRight)
                    .draw()
                    .unwrap();
            }
            || {}
        });
    }

    html! {
        <canvas ref={canvas_ref} width="800" height="600"></canvas>
    }
}
