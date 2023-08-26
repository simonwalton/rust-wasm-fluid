
const colourMaps = {
    blues: [
        [255,255,204],
        [161,218,180],
        [65,182,196],
        [44,127,184],
        [37,52,148]
    ],
    pinks: [
        [241,238,246],
        [215,181,216],
        [223,101,176],
        [221,28,119],
        [152,0,67]
    ]
};

let colourMapSelector = document.getElementById("colourmap"); 
let dtSelector = document.getElementById("dt");

colourMapSelector.addEventListener("change", function() {
    config.colourMap = colourMaps[colourMapSelector.value]
});

dtSelector.addEventListener("change", function() {
    config.dt = +dtSelector.value;
    config.callback();
});

const config = {
    colourMap: colourMaps.blues,
    dt: 0.0001,
    callback: () => {},
}

export default config;
