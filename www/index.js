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

const canvasCoordToArrayAddr = (x, y) => {
    const x0 = parseInt(x / cellSize);
    const y0 = parseInt(y / cellSize);

    return clamp((y0 * fluid.width()) + x0, 0, arraySize-1);
}

var lastX = undefined;
var lastY = undefined;

const mouseClickHandler = (event) => {
    const rect = canvas.getBoundingClientRect();
    const x = event.pageX - rect.x;
    const y = event.pageY - rect.y;
    console.log("mouse move")

    if(lastX && lastY) {
        const sourceU = fluid.source_u();
        const sourceV = fluid.source_v();

        const dx = x - lastX;
        const dy = y - lastY;

        for(var xp=x-8; xp < x+8; xp++) {
            for(var yp=y-8; yp < y+8; yp++) {
                const i = canvasCoordToArrayAddr(xp, yp);
                console.log(xp, yp, i);
                sourceU[i] = dx;
                sourceV[i] = dy;
            }
        }
    }

    lastX = x;
    lastY = y;
}

canvas.addEventListener("mousemove", mouseClickHandler);

const renderLoop = () => {
    fluid.tick();
    drawCells();
        
    setTimeout(() => {
        requestAnimationFrame(renderLoop);
      }, 1000 / 30);
}

const drawCells = () => {
    const cells = fluid.density();
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