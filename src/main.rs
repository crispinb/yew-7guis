mod components;
use components::{Counter, TemperatureConverter};
use log::info;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // gloo is simpler but I couldn't get it to work (the log! macro
    // wasn't resolved)
    info!("worky worky");
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
    // so this completely takes over the html body?
    // see https://github.com/yewstack/yew/tree/master/examples/mount_point for more control over
    // mounting the component
              <>
                  <h2>{"7GUIs"}</h2>
                  <Counter title={"Counter"} />
                  <TemperatureConverter title={"Temperature Converter"}/>
              </>
          }
}
