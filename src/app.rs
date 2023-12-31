/*
    Copyright (c)  Abraão Isvi <avraamisvi@users.noreply.github.com>

    Permission is hereby granted, free of charge, to any
    person obtaining a copy of this software and associated
    documentation files (the "Software"), to deal in the
    Software without restriction, including without
    limitation the rights to use, copy, modify, merge,
    publish, distribute, sublicense, and/or sell copies of
    the Software, and to permit persons to whom the Software
    is furnished to do so, subject to the following
    conditions:

    The above copyright notice and this permission notice
    shall be included in all copies or substantial portions
    of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
    TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
    PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
    SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
    CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
    OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
    IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
 */

use yew::prelude::*;
use web_sys::{Element, Node};
use gloo::utils::document;

use crate::app::{ui::open_side_nav_bar, client::fetch_feature, map::editor_set_add_point_mode};

use self::{_Props::children, ui::init_all_ui, map::load_map};

mod map;
mod ui;
mod client;

#[derive(PartialEq, Properties)]
struct Props {
    children: Html
}

#[function_component]
fn nav(props: &Props) -> Html {
    // memoize as this only needs to be executed once
    let node = {
        let nav_element: Element = document().create_element("nav").unwrap();

        let divs = html! {
            { props.children.clone() }
        };

        if let Html::VRef(div) = divs.clone() {
            let _ = nav_element.append_child(&div);
        }

        let node: Node = nav_element.into();
        Html::VRef(node)   
    };

    // let memoised = use_memo(move |_|{
    //     node
    // }, ());

    // (*memoised).clone()
    node
}

pub struct Main {
    node_ref: NodeRef,
}

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
                <main>
                
                    <nav>
                        <div class="nav-wrapper">
                            <a href="#" class="brand-logo" onclick={Callback::from(|_| {
                                open_side_nav_bar();
                            })}><i class="material-icons">{"menu"}</i></a>
                            <ul id="nav-mobile" class="right hide-on-med-and-down">
                                <li><a href="#" onclick={Callback::from(|_| {
                                editor_set_add_point_mode();
                            })}>{"Add Point"}</a></li>
                                <li><a href="badges.html">{"Components"}</a></li>
                                <li><a href="collapsible.html">{"JavaScript"}</a></li>
                            </ul>
                        </div>
                    </nav>

                    <ul id="slide-out" class="sidenav">
                        <li>
                            <div class="row">
                                <form class="col s12">
                                    <div class="row">
                                        <div class="input-field col s12">
                                        <textarea id="poi-properties" class="materialize-textarea"></textarea>
                                        <label for="textarea1">{"Properties"}</label>
                                        </div>
                                    </div>
                                </form>

                                <button class="btn waves-effect waves-light" type="submit" name="action">
                                    {"Submit"} <i class="material-icons right">{"send"}</i>
                                </button>                                
                            </div>
                        </li>
                        // <li><a href="#!"><i class="material-icons">{"cloud"}</i>{"First Link With Icon"}</a></li>
                        // <li><a href="#!">{"Second Link"}</a></li>
                        // <li><div class="divider"></div></li>
                        // <li><a class="subheader">{"Subheader"}</a></li>
                        // <li><a class="waves-effect" href="#!">{"Third Link With Waves"}</a></li>
                    </ul>
                    // <a href="#" data-target="slide-out" class="sidenav-trigger"><i class="material-icons">{"menu"}</i></a>  

                    <div id="map" style="width: 100%; height: 100vh;"></div>  
                </main>
            }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            init_all_ui();
            load_map();
        }
    }
}


#[function_component(App)]
pub fn app() -> Html {

    html!{
        <Main/>
    }
    
}
