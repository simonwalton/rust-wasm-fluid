import { Fluid } from "wasm-fluid";
import config from "./config"

const fluid = Fluid.new();

const paintBrushSize = 16;

const canvas = document.getElementById("fluid-canvas");
const [canvasWidth, canvasHeight] = [canvas.offsetWidth, canvas.offsetHeight];
const arraySize = fluid.width() * fluid.height();
const [cellSizeX, cellSizeY] = [canvasWidth / fluid.width(), canvasHeight / fluid.height()];
canvas.setAttribute("width", fluid.width());
canvas.setAttribute("height", fluid.height());

const ctx = canvas.getContext('2d');
ctx.imageSmoothingEnabled = true;

const displayImage = ctx.createImageData(fluid.width(), fluid.height());

const clamp = (n, a, b) => Math.min(Math.max(a, n), b);

const fluidCoordToArrayAddr = (x, y) => 
    Math.floor(clamp((Math.floor(y) * fluid.width()) + Math.floor(x), 0, arraySize-1));

const canvasCoordToFluidCoord = (x, y) => [x / cellSizeX, y / cellSizeY];

const lerp = (n, a, b) => a + ((b - a) * n)

const normalisedDensityToColour = (x) => {
    const colourMap = config.colourMap;
    x = clamp(x, 0, 0.999)
    const t = x * (colourMap.length-1);
    const a = Math.floor(t);
    const b = a+1;
    const d = t - a;

    return [0,1,2].map(i => config.colourmapInterpolation ? lerp(d, colourMap[a][i], colourMap[b][i]) : colourMap[a][i]);
}

var lastX = undefined;
var lastY = undefined;
var mouseDown = false;

const distance = (x0,y0,x1,y1) => Math.sqrt((x1-x0)*(x1-x0) + (y1-y0)*(y1-y0));

const mouseClickHandler = (event) => {
    const rect = canvas.getBoundingClientRect();
    const [x, y] = canvasCoordToFluidCoord(event.pageX - rect.x, event.pageY - rect.y);
    const halfBrush = Math.floor(paintBrushSize / 2);
    const density = fluid.d0();

    for(var xp=x-halfBrush; xp < x+halfBrush; xp++) {
        for(var yp=y-halfBrush; yp < y+halfBrush; yp++) {
            let d = distance(xp, yp, x, y);
            if(d < halfBrush) {
                density[fluidCoordToArrayAddr(xp, yp)] = 0.5 * (1 - (d / halfBrush));
            }
        }
    }
}

const mouseMoveHandler = (event) => {
    const rect = canvas.getBoundingClientRect();
    const [x, y] = canvasCoordToFluidCoord(event.pageX - rect.x, event.pageY - rect.y);
    const halfBrush = Math.floor(paintBrushSize / 2);

    if(lastX !== undefined && distance(x, y, lastX, lastY) > 0.01) {
        const dx = x - lastX;
        const dy = y - lastY;
        const sourceU = fluid.source_u();
        const sourceV = fluid.source_v();
        
        for(var xp=x-halfBrush; xp < x+halfBrush; xp++) {
            for(var yp=y-halfBrush; yp < y+halfBrush; yp++) {
                if(distance(xp, yp, x, y) < halfBrush) {
                    const i = fluidCoordToArrayAddr(xp, yp);
                    sourceU[i] = dx;
                    sourceV[i] = dy;
                }
            }
        }

        if(mouseDown) {
            mouseClickHandler(event)
        }
    }

    lastX = x;
    lastY = y;
}

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

canvas.addEventListener("mousemove", mouseMoveHandler);
canvas.addEventListener("click", mouseClickHandler);
canvas.addEventListener("mousedown", (e) => { mouseDown = true; })
canvas.addEventListener("mouseup", (e) => { mouseDown = false; })

config.callback = () => {
    fluid.set_dt(config.dt);
    fluid.set_iterations(config.iterations);
    ctx.imageSmoothingEnabled = false;
}

config.callback();