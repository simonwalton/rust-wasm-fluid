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

    return clamp((y0 * fluid.width()) + x0, 0, arraySize);
}

const mouseClickHandler = (event) => {
    const rect = canvas.getBoundingClientRect();
    const x = event.pageX - rect.x;
    const y = event.pageY - rect.y;

    const sourceU = fluid.source_u();
    sourceU[canvasCoordToArrayAddr(x, y)] += 0.5;

    console.log("add")
}

canvas.addEventListener("mousedown", mouseClickHandler);

const renderLoop = () => {
    fluid.tick();
    drawCells();

    const sourceU = fluid.source_u();
    for(var i = 0; i < arraySize; i++)
        sourceU[i] = 0.0;
        
    console.log("Tick")

    requestAnimationFrame(renderLoop);
}

const drawCells = () => {
    const cells = fluid.density();
    ctx.beginPath();

    console.log(fluid.source_u().slice(0, 2));

    for(var y = 0; y < fluid.height(); y++) {
        for(var x = 0; x < fluid.width(); x++) {
            const colour = cells[(y * fluid.width()) + x];

            ctx.fillStyle = "#" + (new Number(colour * 255)).toString(16).padStart(2, '0') + "0000";
            ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
        }
    }

    ctx.stroke();
}

drawCells();
requestAnimationFrame(renderLoop);