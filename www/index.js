import {Mandelbrot, init_panic_hook} from "mandelbrot";
import {memory} from "mandelbrot/mandelbrot_bg";

const WIDTH = 400;
const HEIGHT = 400;
const panStep = 0.01;
const zoomStep = 0.01;
let upper_left_re = -1;
let upper_left_im = 0.2;
let lower_right_re = -1.2;
let lower_right_im = 0.35;


init_panic_hook();
const mb = Mandelbrot.new(WIDTH, HEIGHT, upper_left_re, upper_left_im, lower_right_re, lower_right_im);
mb.render();

const canvas = document.querySelector("canvas");
canvas.width = WIDTH;
canvas.height = HEIGHT;
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
            ctx.fillRect(col, row, 1, 4)
        }
    }
};

drawCanvas();

document.addEventListener("keydown", function (e) {
    e.preventDefault();
    const key = e.key;

    if (e.ctrlKey) {
        switch (key) {
            case "ArrowDown":
                upper_left_re += zoomStep;
                lower_right_im += zoomStep;
                console.log(upper_left_re, upper_left_im, lower_right_re, lower_right_im);
                break;
            case "ArrowUp":
                upper_left_re -= zoomStep;
                lower_right_im -= zoomStep;
                break;
        }
    } else {
        switch (key) {
            case "ArrowRight":
                upper_left_re -= panStep;
                lower_right_re -= panStep;
                break;
            case "ArrowLeft":
                upper_left_re += panStep;
                lower_right_re += panStep;
                break;
            case "ArrowDown":
                upper_left_im += panStep;
                lower_right_im += panStep;
                break;
            case "ArrowUp":
                upper_left_im -= panStep;
                lower_right_im -= panStep;
                break;
        }
    }


    mb.zoom(upper_left_re, upper_left_im, lower_right_re, lower_right_im);
    mb.render();
    drawCanvas()
});