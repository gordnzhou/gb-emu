import { Emulator } from "gbemulib";
import { memory } from "gbemulib/gbemulib_bg.wasm"

const CANVAS_SCALE = 3;

// in order of: START, SELECT, B, A, DOWN, UP, LEFT, RIGHT.
const KEYMAPPINGS = [
    'i',
    'j',
    'k',
    'l',
    's',
    'w',
    'a',
    'd',
];

const HEIGHT = Emulator.display_height();
const WIDTH = Emulator.display_width();
const DISPLAY_BYTE_LEN = Emulator.display_byte_length();
const AUDIO_OUTPUT_LEN = Emulator.audio_output_length();

let emulator = null;

let key_status = 0xFF;

// INPUT------
window.addEventListener('keydown', function(event) {
    for (let i = 0; i < 8; i++) {
        if (event.key == KEYMAPPINGS[i]) {
            key_status &= ~(1 << (7 - i));
        }
    }
});

window.addEventListener('keyup', function(event) {
    for (let i = 0; i < 8; i++) {
        if (event.key == KEYMAPPINGS[i]) {
            key_status |= 1 << (7 - i)
        }
    }
});
// INPUT------


// DISPLAY------
const canvas = document.getElementById("gameboyDisplayCanvas");
canvas.height = HEIGHT * CANVAS_SCALE;
canvas.width = WIDTH * CANVAS_SCALE;
const ctx = canvas.getContext('2d');

const update_canvas = (display_output_ptr) => {
    const display_output = new Uint8Array(
        memory.buffer, 
        display_output_ptr, 
        WIDTH * HEIGHT * DISPLAY_BYTE_LEN
    );
        
    for (let y = 0; y < HEIGHT; y++) {
        for (let x = 0; x < WIDTH; x++) {
            let index = (y * WIDTH + x) * DISPLAY_BYTE_LEN
            let b = display_output[index];
            let g = display_output[index + 1];
            let r = display_output[index + 2];
            let a = display_output[index + 3] / 255;

            ctx.fillStyle = `rgba(${r}, ${g}, ${b}, ${a})`;
            ctx.fillRect(
                x * CANVAS_SCALE, 
                y * CANVAS_SCALE, 
                CANVAS_SCALE, 
                CANVAS_SCALE
            );
        }
    }
}
// DISPLAY------


const startEmulator = (e) => {
    var arrayBuffer = e.target.result;
    var byteArray = new Uint8Array(arrayBuffer);

    emulator = Emulator.new(byteArray);

    const audioContext = new AudioContext();
    audioContext.resume();
    let audio_node;
    (async function() {
        await audioContext.audioWorklet.addModule('audio.js');
        audio_node = new AudioWorkletNode(audioContext, 'gb-audio-processor');
        audio_node.connect(audioContext.destination);
    })();

    console.log(canvas.height, canvas.width, emulator.game_title());
    
    let main_loop = () => {
        let audio_output_ptr = null;
        let display_output_ptr = null;

        let dur = 0;
        while (audio_output_ptr == null && display_output_ptr == null) {
            if (dur % 4 == 0) {
                emulator.update_joypad(key_status);
            }
            emulator.step();
            audio_output_ptr = emulator.get_audio_output();
            display_output_ptr = emulator.get_display_output();
            dur++;
        }
        
        let timeout_ms = 0;

        if (audio_output_ptr != null && audio_node) {
            const audio_output = new Float32Array(
                memory.buffer,
                audio_output_ptr,
                AUDIO_OUTPUT_LEN
            ) 
            audio_node.port.postMessage(audio_output);
            // timeout_ms += 1000 / 60
        }
    
        if (display_output_ptr != null) {
            update_canvas(display_output_ptr);
            timeout_ms += 1000 / 60
        }

        setTimeout(main_loop, timeout_ms)
    }  

    main_loop();
}

document.getElementById('fileInput').addEventListener('change', function(e) {
    var file = e.target.files[0];
    if (!file) {
        console.log("No file selected");
        return;
    }

    var reader = new FileReader();
    reader.readAsArrayBuffer(file);
    reader.onload = function(e) {
        startEmulator(e)
    };
});
