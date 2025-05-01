use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use plotters_canvas::CanvasBackend;
use web_sys::{HtmlCanvasElement, window};
use plotters::prelude::*;
use chrono::{ DateTime, Utc };
use std::ops::Range;
use std::rc::Rc;
use crate::app::utils::time_formatter::smart_time_label;

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
    pub data: Rc<Vec<DataPoint>>,
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
#[derive(Properties, Clone, PartialEq)]
pub struct TimeSeriesChartProps {
    pub config: Rc<TimeSeriesChartConfig>,
}

pub enum Msg {
    Redraw,
}

pub struct TimeSeriesChart { 
    props: TimeSeriesChartProps,
    canvas_ref: NodeRef,
    raf_closure: Option<Closure<dyn FnMut()>>,
}

impl Component for TimeSeriesChart {
    type Message = Msg;
    type Properties = TimeSeriesChartProps;

    fn create(ctx: &Context<Self>) -> Self {
        TimeSeriesChart {
            props: ctx.props().clone(),
            canvas_ref: NodeRef::default(),
            raf_closure: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, previous_props: &TimeSeriesChartProps) -> bool {
        if previous_props != ctx.props() {
            self.props = ctx.props().clone();
            ctx.link().send_message(Msg::Redraw);
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas
                ref={self.canvas_ref.clone()}
                style="width:100%; height:100%; display:block;"
            />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        ctx.link().send_message(Msg::Redraw);
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Redraw => {
                if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
                    let canvas_clone = canvas.clone();
                    let config_rc = Rc::clone(&self.props.config);

                    let draw_canvas_backend = Closure::wrap(Box::new(move || {
                        let width = canvas_clone.client_width() as u32;
                        let height = canvas_clone.client_height() as u32;

                        if width == 0 || height == 0 { return; }

                        let device_pixel_ratio = window().unwrap().device_pixel_ratio();

                        let canvas_clone = canvas_clone.clone();
                        
                        let scaling_factor = 1.7;

                        canvas_clone.set_width((width as f64 * device_pixel_ratio * scaling_factor) as u32);
                        canvas_clone.set_height((height as f64 * device_pixel_ratio * scaling_factor) as u32); 

                        let backend = CanvasBackend::with_canvas_object(canvas_clone).expect("backend");
                        let root = backend.into_drawing_area();
                        root.fill(&RGBColor(15, 15, 25)).unwrap();

                        let config = &*config_rc;

                        let mut chart = ChartBuilder::on(&root)
                        .margin(20)
                        .set_label_area_size(LabelAreaPosition::Left, width / 6)
                        .set_label_area_size(LabelAreaPosition::Bottom, height / 6)
                        .set_label_area_size(LabelAreaPosition::Right, 0)
                        .set_label_area_size(LabelAreaPosition::Top, 0)
                        .caption(&config.caption, ("sans-serif", (height as f32 * 0.08) as u32).into_font().color(&WHITE))
                        .build_cartesian_2d(config.x_range.clone(), config.y_range.clone())
                        .unwrap();

                        chart.configure_mesh()
                        .x_labels(config.x_labels)
                        .y_labels(10)
                        .x_desc(&config.x_desc)
                        .y_desc(&config.y_desc)
                        .axis_style(&WHITE.mix(0.9))
                        .axis_desc_style(("sans-serif", (height as f32 * 0.08) as u32).into_font().color(&WHITE))
                        .light_line_style(&WHITE.mix(0.15))
                        .bold_line_style(&WHITE.mix(0.25))
                        .label_style(("sans-serif", (height as f32 * 0.06) as u32).into_font().color(&WHITE))
                        .x_label_formatter(&|datetime: &DateTime<Utc>| smart_time_label(datetime))
                        .y_label_formatter(&|y| format!("{:.1}", y))
                        .draw()
                        .unwrap();
                        
                        
                        for series in &config.series {

                            let style = ShapeStyle::from(&series.color).stroke_width(3);

                            chart.draw_series(LineSeries::new(
                                series.data.iter().map(|point| (point.timestamp, point.value)),
                                style.clone()
                            )).unwrap()
                            .label(&series.label)
                            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 50, y)], style.clone()));

                        }

                        chart.configure_series_labels()
                        .legend_area_size(width / 12)
                        .background_style(&RGBColor(15, 15, 25).mix(0.8))
                        .border_style(&WHITE.mix(0.5))
                        .label_font(("sans-serif", (height as f32 * 0.06) as u32).into_font().color(&WHITE))
                        .position(SeriesLabelPosition::UpperRight)
                        .margin(15)
                        .draw()
                        .unwrap();

                    }) as Box<dyn FnMut()>);

                    window().unwrap().request_animation_frame(draw_canvas_backend.as_ref().unchecked_ref()).unwrap();

                    self.raf_closure = Some(draw_canvas_backend);
                }
                false
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        self.raf_closure = None;
    }
}