import { Emulator } from "gbemulib";
import { memory } from "gbemulib/gbemulib_bg.wasm";

export const GBDisplay = (() => {
    const WIDTH = Emulator.display_width();
    const HEIGHT = Emulator.display_height();
    const DISPLAY_BYTE_LEN = Emulator.display_byte_length();

    const CANVAS_SCALE = 3;
    const CLEAR_COLOUR = "#FFFFE8";

    const canvas = document.getElementById("gb-display");

    canvas.height = HEIGHT * CANVAS_SCALE;
    canvas.width = WIDTH * CANVAS_SCALE;

    const ctx = canvas.getContext('2d');

    ctx.imageSmoothingEnabled = false;
    ctx.scale(CANVAS_SCALE, CANVAS_SCALE);

    return {
        updateCanvas: (displayOutputPtr) => {
            const display_output = new Uint8Array(
                memory.buffer, 
                displayOutputPtr, 
                WIDTH * HEIGHT * DISPLAY_BYTE_LEN
            );
            
            let tempCanvas = document.createElement('canvas');
            tempCanvas.width = WIDTH;
            tempCanvas.height = HEIGHT;
            let tempCtx = tempCanvas.getContext('2d');
            let imageData = ctx.createImageData(WIDTH, HEIGHT);
            let data = imageData.data;
    
            let i = 0;
            for (let y = 0; y < HEIGHT; y++) {
                for (let x = 0; x < WIDTH; x++) {
                    let index = (y * WIDTH + x) * DISPLAY_BYTE_LEN
                    data[i++] = display_output[index + 2];
                    data[i++] = display_output[index + 1];
                    data[i++] = display_output[index];
                    data[i++] = display_output[index + 3];
                }
            }
    
            tempCtx.putImageData(imageData, 0, 0);
            ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
            ctx.drawImage(tempCanvas, 0, 0);
        },

        clearCanvas: () => {   
            ctx.fillStyle = CLEAR_COLOUR;
            ctx.fillRect(0, 0, canvas.width, canvas.height);
        }
    }
})();