#![allow(non_snake_case)]
/**
* Copyright (c)  Abraão Isvi <avraamisvi@users.noreply.github.com>
* 
* Permission is hereby granted, free of charge, to any
* person obtaining a copy of this software and associated
* documentation files (the "Software"), to deal in the
* Software without restriction, including without
* limitation the rights to use, copy, modify, merge,
* publish, distribute, sublicense, and/or sell copies of
* the Software, and to permit persons to whom the Software
* is furnished to do so, subject to the following
* conditions:
* 
* The above copyright notice and this permission notice
* shall be included in all copies or substantial portions
* of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
* ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
* TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
* PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
* SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
* CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
* OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
* IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
* DEALINGS IN THE SOFTWARE. 
**/

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::{RequestInit, RequestMode, Request, Response, Headers, wasm_bindgen::JsValue};
use wasm_bindgen_futures::{JsFuture, spawn_local};

fn main() {
    // launch the web app
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {

    let create_eval = use_eval(cx);
    let poi_properties = use_state(cx, || String::from(""));

    cx.render(rsx! {
        div {
            onmounted: move |_| {
                let mut eval = create_eval(
                    r#"
                    load_map();
                    M.AutoInit();
                    "#,
                ).unwrap();

                let future = use_future(cx, (), |_| {
                    to_owned![eval];
                    async move {
                        // You can receive any message from JavaScript with the recv method
                        eval.join().await.unwrap()
                    }
                });
                
                future.value();                            
            },

            nav {
                div { class: "nav-wrapper",
                    a { onclick: |_| {
                            let eval = create_eval(
                                r#"
                                    var elem = document.querySelector('.sidenav');
                                    var instance = M.Sidenav.getInstance(elem);
                                    instance.open();
                                "#,
                            ).unwrap();
            
                            let future = use_future(cx, (), |_| {
                                to_owned![eval];
                                async move {
                                    // You can receive any message from JavaScript with the recv method
                                    eval.join().await.unwrap()
                                }
                            });
                            
                            future.value();   
                        },
                        href: "#", class: "brand-logo", i { class: "material-icons", "menu" } 
                    }
                    ul { class: "right hide-on-med-and-down", id: "nav-mobile",
                        li { a {
                                onclick: |_| {
                                    let eval = create_eval(
                                        r#"
                                            editor_set_add_point_mode();
                                        "#,
                                    ).unwrap();
                    
                                    let future = use_future(cx, (), |_| {
                                        to_owned![eval];
                                        async move {
                                            // You can receive any message from JavaScript with the recv method
                                            eval.join().await.unwrap()
                                        }
                                    });
                                    
                                    future.value();   
                                },                             
                            href: "#", "Add Point" 
                        } }
                        li { a { href: "#", "Add Circle" } }
                        li { a { href: "#", "Add Polygon" } }
                    }
                }
            },

            ul {class: "sidenav", id: "slide-out",
                li {
                    div { class: "row",
                        form { class: "col s12",
                            
                            div { class: "row",
                                div { class: "input-field col s12",
                                    label { r#for: "poi-lng", "Longitude"}
                                    input {name: "lng",  value: "", id: "poi-lng", readonly: true}
                                }
                            }
                            div { class: "row",
                                div { class: "input-field col s12",
                                    label { r#for: "poi-lat", "Latitude" }
                                    input { value: "", name: "lat", readonly: true, id: "poi-lat" }
                                }
                            }                            
                            div { class: "row",
                                div { class: "input-field col s12",
                                    textarea { onchange: |evt| {
                                        poi_properties.set(evt.value.clone());
                                    },
                                    class: "materialize-textarea", id: "poi-properties" }
                                    label { r#for: "poi-properties", "Properties" }
                                }
                            }
                        }
                        button {
                            onclick: move |_| {
                                    let eval = create_eval(
                                        r#"
                                            var poi_lng = document.querySelector('#poi-lng');
                                            var poi_lat = document.querySelector('#poi-lat');
                                            var poi_properties = document.querySelector('#poi-properties');

                                            dioxus.send({
                                                lat: parseFloat(poi_lat.value),
                                                lng: parseFloat(poi_lng.value),
                                                properties: JSON.parse(poi_properties.value) 
                                            });
                                        "#,
                                    ).unwrap();

                                    let value = futures::executor::block_on(async {
                                        eval.recv().await.unwrap() 
                                    });

                                    cx.spawn(async move {
                                        let result = save_feature(&value).await;
                                        log::debug!("{:?}", result);
                                    });


                            },
                            name: "action",
                            r#type: "button",
                            class: "btn waves-effect waves-light",
                            "Save "
                            i { class: "material-icons right", "send" }
                        }
                    }
                }
            },

            div {
                id: "map",
                style: "width: 100%; height: 100vh;"
            }
        }
    })
}

async fn save_feature(value: &Value) {

    let content = JsValue::from(r#"
            {
                "type": "Feature",
                "geometry": {
                "type": "point",
                "coordinates": [
                    -38.520302,
                    -12.999122
                ]
                },
                "properties": {
                "some": "ABACATE"
                }
            }    
    "#);

    // let headers = JsValue::from("Content-Type: application/json");

    let headers = Headers::new().unwrap();
    // headers.set("Accept", "application/json").unwrap();
    // headers.set("Authorization", &bearer).unwrap();
    headers.set("Content-Type", "application/json").unwrap();

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&content));
    opts.headers(&headers);


    let request = Request::new_with_str_and_init("http://127.0.0.1:8000/collections/1/item", &opts).unwrap();
    let window = web_sys::window().unwrap(); 
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;

    match resp_value {
        Ok(js_value) => {
            let response = Response::from(js_value);
            let json_promise = response.json().unwrap();

            let json_result = JsFuture::from(json_promise).await;

            match json_result {
                Ok(json) => {
                    log::debug!("{json:?}");
                },
                Err(err) => {
                    log::error!("{err:?}");
                }
            }
        },
        Err(err) => {
            log::error!("{err:?}");
        }
    };
}