import { Fluid } from "wasm-fluid";
import { memory } from "wasm-fluid/wasm_fluid_bg"

const fluid = Fluid.new();

const paintBrushSize = 16;

const canvas = document.getElementById("fluid-canvas");
const [canvasWidth, canvasHeight] = [canvas.offsetWidth, canvas.offsetHeight];
const arraySize = fluid.width() * fluid.height();
const [cellSizeX, cellSizeY] = [canvasWidth / fluid.width(), canvasHeight / fluid.height()];
canvas.setAttribute("width", fluid.width());
canvas.setAttribute("height", fluid.height());

const colourMap = [
    [255,255,217],
    [255,255,217],
    [255,255,217],
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
    Math.floor(clamp((Math.floor(y) * fluid.width()) + Math.floor(x), 0, arraySize-1));

const canvasCoordToFluidCoord = (x, y) => [x / cellSizeX, y / cellSizeY];

var lastX = undefined;
var lastY = undefined;
var mouseDown = false;

const distance = (x0,y0,x1,y1) => Math.sqrt((x1-x0)*(x1-x0) + (y1-y0)*(y1-y0));

const mouseMoveHandler = (event) => {
    const rect = canvas.getBoundingClientRect();
    const [x, y] = canvasCoordToFluidCoord(event.pageX - rect.x, event.pageY - rect.y);
    const halfBrush = Math.floor(paintBrushSize / 2);

    if(lastX !== undefined && distance(x, y, lastX, lastY) > 0.3) {
        const dx = x - lastX;
        const dy = y - lastY;
        const sourceU = fluid.source_u();
        const sourceV = fluid.source_v();
        
        for(var xp=x-halfBrush; xp < x+halfBrush; xp++) {
            for(var yp=y-halfBrush; yp < y+halfBrush; yp++) {
                const i = fluidCoordToArrayAddr(xp, yp);
                sourceU[i] = dx;
                sourceV[i] = dy;
            }
        }

        if(mouseDown) {
            mouseClickHandler(event)
        }
    }

    lastX = x;
    lastY = y;
}

const mouseClickHandler = (event) => {
    const rect = canvas.getBoundingClientRect();
    const [x, y] = canvasCoordToFluidCoord(event.pageX - rect.x, event.pageY - rect.y);
    const halfBrush = Math.floor(paintBrushSize / 2);
    const density = fluid.d0();

    for(var xp=x-halfBrush; xp < x+halfBrush; xp++) {
        for(var yp=y-halfBrush; yp < y+halfBrush; yp++) {
            density[fluidCoordToArrayAddr(xp, yp)] = 1.0;
        }
    }
}

const lerp = (n, a, b) => a + ((b - a) * n)

const normalisedDensityToColour = (x) => {
    x = clamp(x, 0, 0.999)
    const t = x * (colourMap.length-1);
    const a = Math.floor(t);
    const b = a+1;
    const d = t - a;

    return [0,1,2].map(i => lerp(d, colourMap[a][i], colourMap[b][i]));
}

canvas.addEventListener("mousemove", mouseMoveHandler);
canvas.addEventListener("click", mouseClickHandler);
canvas.addEventListener("mousedown", (e) => { mouseDown = true; })
canvas.addEventListener("mouseup", (e) => { mouseDown = false; })

const renderLoop = () => {
    fluid.tick();
    drawCells();
        
    requestAnimationFrame(renderLoop);
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