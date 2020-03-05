import {Mandelbrot, init_panic_hook} from "mandelbrot";
import {memory} from "mandelbrot/mandelbrot_bg";

const WIDTH = 200;
const HEIGHT = 200;

init_panic_hook();
const mb = Mandelbrot.new(WIDTH, HEIGHT, -1.2, 0.35, -1, 0.2);
mb.render();

const canvas = document.querySelector("canvas");
const ctx = canvas.getContext("2d");
const canvasImageData = ctx.createImageData(canvas.width, canvas.height);

// connect to the linear memory of wasm
// Pointer to the pixels defined in rust.
const pixelPtr = mb.pixels();
// shared linear memory
const pixels = new Uint8Array(memory.buffer, pixelPtr, length = WIDTH * HEIGHT);

const drawCanvas = () => {
    for (let row = 0; row < HEIGHT; row++) {
        for (let col = 0; col < WIDTH; col++) {
            const idx = row * HEIGHT + col;
            const pixelVal = pixels[idx];
            ctx.fillStyle = `rgb(${pixelVal}, ${pixelVal}, ${pixelVal})`;
            ctx.fillRect(row, col, 1, 1)
        }
    }
};

drawCanvas();