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

#[wasm_bindgen]
extern "C" {
    pub fn add_source(collection: JsValue);
    pub fn add_layer(layer: JsValue);
    pub fn load_map();
}

// pub fn execute_load_map() {

//     let json = futures::executor::block_on(async {
//         fetch_feature().await
//     });

//     let collection = json.as_str();

//     add_feature(collection)
//     // add_feature(r#"
//     //     {
//     //         "type": "Feature",
//     //         "geometry": {
//     //             "type": "Polygon",
//     //             "coordinates": [
//     //                 [
//     //                     [-67.13734351262877, 45.137451890638886],
//     //                     [-66.96466, 44.8097],
//     //                     [-68.03252, 44.3252],
//     //                     [-69.06, 43.98],
//     //                     [-70.11617, 43.68405],
//     //                     [-70.64573401557249, 43.090083319667144],
//     //                     [-70.75102474636725, 43.08003225358635],
//     //                     [-70.79761105007827, 43.21973948828747],
//     //                     [-70.98176001655037, 43.36789581966826],
//     //                     [-70.94416541205806, 43.46633942318431],
//     //                     [-71.08482, 45.3052400000002],
//     //                     [-70.6600225491012, 45.46022288673396],
//     //                     [-70.30495378282376, 45.914794623389355],
//     //                     [-70.00014034695016, 46.69317088478567],
//     //                     [-69.23708614772835, 47.44777598732787],
//     //                     [-68.90478084987546, 47.184794623394396],
//     //                     [-68.23430497910454, 47.35462921812177],
//     //                     [-67.79035274928509, 47.066248887716995],
//     //                     [-67.79141211614706, 45.702585354182816],
//     //                     [-67.13734351262877, 45.137451890638886]
//     //                 ]
//     //             ]
//     //         }
//     //     }
//     // "#)
// }