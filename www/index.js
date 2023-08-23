import { Fluid } from "wasm-fluid";
import { memory } from "wasm-fluid/wasm_fluid_bg"

const fluid = Fluid.new();
const cellSize = 8;

const paintBrushSize = 16;

const canvas = document.getElementById("fluid-canvas");
canvas.width = fluid.width() * cellSize;
canvas.height = fluid.height() * cellSize;
const arraySize = fluid.width() * fluid.height();

const colourMap = ['#ffffd9','#edf8b1','#c7e9b4','#7fcdbb','#41b6c4','#1d91c0','#225ea8','#0c2c84'];

const ctx = canvas.getContext('2d');

const clamp = (n, a, b) => Math.min(Math.max(a, n), b);

const fluidCoordToArrayAddr = (x, y) => 
    parseInt(clamp((Math.floor(y) * fluid.width()) + Math.floor(x), 0, arraySize-1));

const canvasCoordToFluidCoord = (x, y) => [x / cellSize, y / cellSize];

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
    ctx.beginPath();

    for(var y = 0; y < fluid.height(); y++) {
        for(var x = 0; x < fluid.width(); x++) {
            const cell = cells[(y * fluid.width()) + x];

            ctx.fillStyle = normalisedDensityToColour(cell);
            ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
        }
    }

    ctx.stroke();
}

drawCells();
requestAnimationFrame(renderLoop);