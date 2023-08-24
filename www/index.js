import { Fluid } from "wasm-fluid";
import { memory } from "wasm-fluid/wasm_fluid_bg"

const fluid = Fluid.new();

const paintBrushSize = 16;

const canvas = document.getElementById("fluid-canvas");
const [canvasWidth, canvasHeight] = [canvas.offsetWidth, canvas.offsetHeight];
const arraySize = fluid.width() * fluid.height();
const [cellSizeX, cellSizeY] = [canvasWidth / fluid.width(), canvasHeight / fluid.height()];

const colourMap = [
    [255,255,217],
    [237,248,177],
    [199,233,180],
    [127,205,187],
    [65,182,196],
    [29,145,192],
    [34,94,168],
    [12,44,132]
];

const ctx = canvas.getContext('2d');
ctx.imageSmoothingEnabled = true;

const displayImage = ctx.createImageData(fluid.width(), fluid.height());

const clamp = (n, a, b) => Math.min(Math.max(a, n), b);

const fluidCoordToArrayAddr = (x, y) => 
    parseInt(clamp((Math.floor(y) * fluid.width()) + Math.floor(x), 0, arraySize-1));

const canvasCoordToFluidCoord = (x, y) => [x / cellSizeX, y / cellSizeY];

var lastX = undefined;
var lastY = undefined;
var mouseDown = false;

const mouseMoveHandler = (event) => {
    const rect = canvas.getBoundingClientRect();
    const [x, y] = canvasCoordToFluidCoord(event.pageX - rect.x, event.pageY - rect.y);
    const halfBrush = parseInt(paintBrushSize / 2);

    if(lastX && lastY) {
        const sourceU = fluid.source_u();
        const sourceV = fluid.source_v();

        const dx = x - lastX;
        const dy = y - lastY;

        for(var xp=x-halfBrush; xp < x+halfBrush; xp++) {
            for(var yp=y-halfBrush; yp < y+halfBrush; yp++) {
                const i = fluidCoordToArrayAddr(xp, yp);
                sourceU[i] += clamp(dx, -1, 1)
                sourceV[i] += clamp(dy, -1, 1);
            }
        }
    }

    if(mouseDown) {
        mouseClickHandler(event)
    }

    lastX = x;
    lastY = y;
}

const mouseClickHandler = (event) => {
    const rect = canvas.getBoundingClientRect();
    const [x, y] = canvasCoordToFluidCoord(event.pageX - rect.x, event.pageY - rect.y);
    console.log(event.pageX - rect.x, event.pageY - rect.y);
    console.log(x,y);
    const halfBrush = parseInt(paintBrushSize / 2);
    const density = fluid.d0();

    for(var xp=x-halfBrush; xp < x+halfBrush; xp++) {
        for(var yp=y-halfBrush; yp < y+halfBrush; yp++) {
            density[fluidCoordToArrayAddr(xp, yp)] = 1.0;
        }
    }
}

const normalisedDensityToColour = (x) => {
    const idx = parseInt(clamp(x * colourMap.length, 0.0, colourMap.length-1));
    return colourMap[idx];
}

canvas.addEventListener("mousemove", mouseMoveHandler);
canvas.addEventListener("click", mouseClickHandler);
canvas.addEventListener("mousedown", (e) => { mouseDown = true; })
canvas.addEventListener("mouseup", (e) => { mouseDown = false; })

const renderLoop = () => {
    fluid.tick();
    drawCells();
        
    setTimeout(() => {
        requestAnimationFrame(renderLoop);
      }, 1000 / 60);
}

const drawCells = () => {
    const cells = fluid.d();
    const N = fluid.width();

    var p = 0; 
    for(var i = 0; i < arraySize; i++) { 
        let [r,g,b] = normalisedDensityToColour(cells[i]);
        displayImage.data[p++] = r;
        displayImage.data[p++] = g;
        displayImage.data[p++] = b;
        displayImage.data[p++] = 255;
    }

    ctx.putImageData(displayImage, 0, 0);
}

drawCells();
requestAnimationFrame(renderLoop);