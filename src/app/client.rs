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

 use wasm_bindgen::prelude::*;
 use wasm_bindgen_futures::{JsFuture, spawn_local};
 use web_sys::{Request, RequestInit, RequestMode, Response};

use super::map::{add_source, add_layer};
 
//  #[wasm_bindgen]
//  extern "C" {
//      fn add_source();
//      fn add_source();
//  }

// pub struct PointLayout {
//     pub id: String,
//     pub r#type: String,
//     pub source: String,
//     pub layout: 
// }

// #[derive(Serialize, Deserialize)]
// pub struct PointLayer {
//     pub id: String,
//     pub r#type: String,
//     pub source: String,
//     pub layout: 
// }
 
 pub fn fetch_feature() {    
     spawn_local(async {
 
         let mut opts = RequestInit::new();
         opts.method("GET");
         opts.mode(RequestMode::Cors);
     
         let url = "http://127.0.0.1:8000/collections/9/items/41";
     
         let request = Request::new_with_str_and_init(&url, &opts).unwrap();
     
         let result_accept_header = request
             .headers()
             .set("Accept", "application/json").unwrap();
 
 
         let window = web_sys::window().unwrap(); 
         let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
 
         match resp_value {
             Ok(js_value) => {
 
                 let response = Response::from(js_value);
 
                 let json_res = response.json();
 
                 match json_res {
                     Ok(json_promise) => {
                         let json_result = JsFuture::from(json_promise).await;
                        //  let layer = JsValue::from(value);//TODO From Json
                        

                         match json_result {
                             Ok(json) => {
                                add_source("test".to_string(), json);
                                add_layer(r#"
                                    {
                                        'id': 'points',
                                        'type': 'symbol',
                                        'source': 'point',
                                        'layout': {
                                            'icon-image': 'circle',
                                            'icon-size': 0.25
                                        }
                                    }
                                "#.into());
                             },
                             Err(err) => web_sys::console::log_1(&err),
                         }
                         
                     },
                     Err(err) => {
                         web_sys::console::log_1(&err);
                     },
                 }
             },
             Err(err) => {
                 let err_str = err.as_string().unwrap();
                 let js_value = JsValue::from(&err_str);
                 web_sys::console::log_1(&js_value)
             }
         }
     })
 }
 