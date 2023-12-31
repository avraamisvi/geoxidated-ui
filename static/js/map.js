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

var editor_modes = {
    ADD_POINT: 1,
    ADD_POLYGON: 2,
    ADD_CIRCLE: 3,
    NONE: 0
}

var editor_status = {
    mode: editor_modes.NONE
}

function editor_set_add_point_mode() {
    editor_status.mode = editor_modes.ADD_POINT;
}

function editor_set_none_mode() {
    editor_status.mode = editor_modes.NONE;
}
  
var map = {};

function load_map() {
    map = new maplibregl.Map({
        container: 'map', // container id
        style: 'https://demotiles.maplibre.org/style.json', // style URL
        center: [0, 0], // starting position [lng, lat]
        zoom: 1 // starting zoom
    });

    map.on('load', () => {

        const canvas = map.getCanvasContainer();

        map.loadImage('/imgs/circle.png', (error, image) => {
              if (error) throw error;
              map.addImage('circle', image);
          } )      
          
          map.loadImage('/imgs/pin.png', (error, image) => {
              if (error) throw error;
              map.addImage('pin', image);
          } )    

          //testing ideas
          map.addSource('points', {
              'type': 'geojson',
              'data': null
          });

          map.addLayer({
                  'id': 'points',
                  'type': 'symbol',
                  'source': 'points',
                  // 'paint': {
                  //     'circle-radius': 10,
                  //     'circle-color': '#3887be'
                  // }
                  'layout': {
                      'icon-image': 'pin',
                      'icon-size': 0.7
                  }
              });

              map.on('click', 'points', (event) => {

                if(editor_status.mode == editor_modes.ADD_POINT) {
                  add_point(event);
                }

                console.log(`map click:`);
                console.log(event.features);

                editor_set_none_mode();
              });

              map.on('click', (event) => {

                if(editor_status.mode == editor_modes.ADD_POINT) {
                  add_point(event);
                }

                editor_set_none_mode();
              });

              map.on('mouseenter', 'points', () => {            
                canvas.style.cursor = 'move';
              });

              map.on('mouseleave', 'points', () => {
                canvas.style.cursor = '';
              });   

      })
}

function add_point(event) {

    if(event.features && event.features.length > 0) {
        console.log("Show features attr")
      } else {

        let points_source = map.getSource('points');

        points_source.setData({
          "type": "Feature",
          "geometry": {
            "type": "Point",
            "coordinates": [event.lngLat.lng, event.lngLat.lat]
          }
        })
      }        
  }

  function add_source(name, type, collection) {
    console.log(collection);
    map.addSource(name, 
    {
        'type': type,
        'data': collection.features[0]
    });        
  }

  function add_layer(layer) {
    map.addLayer(JSON.parse(layer));           
  }