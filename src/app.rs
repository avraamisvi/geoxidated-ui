use yew::prelude::*;
use web_sys::HtmlBodyElement;

use crate::app::map::execute_load_map;

mod map;

#[function_component(App)]
pub fn app() -> Html {

    html! {
        <main onclick={Callback::from(|_| {
                execute_load_map()
            })}>
            <div id="map" style="width: 100%; height: 100vh;"></div>
        </main>
    }
}
