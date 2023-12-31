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

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {

    let create_eval = use_eval(cx);

    cx.render(rsx! {
        div {
            onmounted: move |_| {
                let mut eval = create_eval(
                    r#"
                    load_map();
                    M.AutoInit();
                    console.log("loaded with dioxus");
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
                            input { name: "lng", r#type: "hidden", value: "", id: "poi-lng" }
                            input { value: "", name: "lat", r#type: "hidden", id: "poi-lat" }
                            div { class: "row",
                                div { class: "input-field col s12",
                                    textarea { class: "materialize-textarea", id: "poi-properties" }
                                    label { r#for: "textarea1", "Properties" }
                                }
                            }
                        }
                        button {
                            name: "action",
                            r#type: "submit",
                            class: "btn waves-effect waves-light",
                            "Submit "
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