import { Fluid } from "wasm-fluid";
import { memory } from "wasm-fluid/wasm_fluid_bg"

const fluid = Fluid.new();
const cellSize = 8;

const canvas = document.getElementById("fluid-canvas");
canvas.width = fluid.width() * cellSize;
canvas.height = fluid.height() * cellSize;
const arraySize = fluid.width() * fluid.height();

const ctx = canvas.getContext('2d');

const clamp = (n, a, b) => Math.min(Math.max(a, n), b);

const fluidCoordToArrayAddr = (x, y) => 
    parseInt(clamp((Math.floor(y) * fluid.width()) + Math.floor(x), 0, arraySize-1));

const canvasCoordToFluidCoord = (x, y) => [x / cellSize, y / cellSize];

var lastX = undefined;
var lastY = undefined;

const mouseMoveHandler = (event) => {
    const rect = canvas.getBoundingClientRect();
    const [x, y] = canvasCoordToFluidCoord(event.pageX - rect.x, event.pageY - rect.y);

    if(lastX && lastY) {
        const sourceU = fluid.source_u();
        const sourceV = fluid.source_v();

        const dx = x - lastX;
        const dy = y - lastY;

        for(var xp=x-2; xp < x+2; xp++) {
            for(var yp=y-2; yp < y+2; yp++) {
                const i = fluidCoordToArrayAddr(xp, yp);
                sourceU[i] += clamp(dx, -1, 1)
                sourceV[i] += clamp(dy, -1, 1);
            }
        }
    }

    lastX = x;
    lastY = y;
}

const mouseClickHandler = (event) => {
    const rect = canvas.getBoundingClientRect();
    const [x, y] = canvasCoordToFluidCoord(event.pageX - rect.x, event.pageY - rect.y);

    const density = fluid.d0();

    for(var xp=x-2; xp < x+2; xp++) {
        for(var yp=y-2; yp < y+2; yp++) {
            density[fluidCoordToArrayAddr(xp, yp)] = 1.0;
        }
    }
}

canvas.addEventListener("mousemove", mouseMoveHandler);
canvas.addEventListener("click", mouseClickHandler);

const renderLoop = () => {
    fluid.tick();
    drawCells();
        
    setTimeout(() => {
        requestAnimationFrame(renderLoop);
      }, 1000 / 30);
}

const drawCells = () => {
    const cells = fluid.d();
    ctx.beginPath();

    for(var y = 0; y < fluid.height(); y++) {
        for(var x = 0; x < fluid.width(); x++) {
            const cell = cells[(y * fluid.width()) + x];
            const colour = parseInt(cell * 255);

            ctx.fillStyle = "#" + colour.toString(16).padStart(2, '0') + "0000";
            ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
        }
    }

    ctx.stroke();
}

drawCells();
requestAnimationFrame(renderLoop);