
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
    ],
    redblue: [
        [215,48,39],
        [244,109,67],
        [253,174,97],
        [254,224,144],
        [224,243,248],
        [171,217,233],
        [116,173,209],
        [69,117,180],
    ],
    colourful: [
        [158,1,66],
        [213,62,79],
        [244,109,67],
        [253,174,97],
        [254,224,139],
        [230,245,152],
        [171,221,164],
        [102,194,165],
        [50,136,189],
        [94,79,162]
    ],
    pastel: [
        [141,211,199],
        [255,255,179],
        [190,186,218],
        [190,186,218],
        [128,177,211],
        [253,180,98],
        [179,222,105],
        [252,205,229]
    ]
};

const gridResolutions = {
    low: 100,
    medium: 200,
    high: 300,
    veryhigh: 500
}

let colourMapSelector = document.getElementById("colourmap"); 
Object.entries(colourMaps).forEach(obj => {
    let option = document.createElement("option");
    option.value = obj[0];
    option.text = obj[0];
    colourMapSelector.add(option);
});
colourMapSelector.addEventListener("change", function() {
    config.colourMap = colourMaps[colourMapSelector.value]
});

let dtSelector = document.getElementById("dt");
dtSelector.addEventListener("change", function() {
    config.dt = +dtSelector.value;
    config.callback();
});

let colourmapInterpolationSelector = document.getElementById("colourmapInterpolation");
colourmapInterpolationSelector.addEventListener("change", function() {
    config.colourmapInterpolation = colourmapInterpolationSelector.checked;
    config.callback();
});

let iterationsSelector = document.getElementById("iterations");
iterationsSelector.addEventListener("change", function() {
    config.iterations = +iterationsSelector.value;
    config.callback();
});

let resolutionSelector = document.getElementById("resolution");
Object.entries(gridResolutions).forEach(obj => {
    let option = document.createElement("option");
    option.value = +obj[1];
    option.text = obj[0];
    resolutionSelector.add(option);
});
resolutionSelector.addEventListener("change", function() {
    config.resolution = +resolutionSelector.value;
    config.callback(true);
});

const config = {
    colourMap: colourMaps.blues,
    dt: 0.0001,
    colourmapInterpolation: true,
    resolution: 100,
    iterations: 10,
    callback: (refresh = false) => {},
}

export default config;
