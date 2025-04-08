pub mod app;
mod app_main;

use app_main::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
