import { Emulator } from "gbemulib";
import { memory } from "gbemulib/gbemulib_bg.wasm"

const CANVAS_SCALE = 3;

// in order of: START, SELECT, B, A, DOWN, UP, LEFT, RIGHT.
const KEYMAPPINGS = [
    'Enter',
    'ShiftRight',
    'z',
    'x',
    'ArrowDown',
    'ArrowUp',
    'ArrowLeft',
    'ArrowRight',
];

const HEIGHT = Emulator.display_height();
const WIDTH = Emulator.display_width();
const DISPLAY_BYTE_LEN = Emulator.display_byte_length();
const AUDIO_OUTPUT_LEN = Emulator.audio_output_length();

// INPUT------
let key_status = 0xFF;

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
const canvas = document.getElementById("gb-display");
canvas.height = HEIGHT * CANVAS_SCALE;
canvas.width = WIDTH * CANVAS_SCALE;
const ctx = canvas.getContext('2d');
const CLEAR_COLOUR = "#FFFFE8";

ctx.imageSmoothingEnabled = false;
ctx.scale(CANVAS_SCALE, CANVAS_SCALE);

const updateCanvas = (displayOutputPtr) => {
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
}

const clearCanvas = () => {   
    for (let y = 0; y < HEIGHT; y++) {
        for (let x = 0; x < WIDTH; x++) {
            ctx.fillStyle = CLEAR_COLOUR;
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


// AUDIO------
let audioContext;
let audioNode;
let audioVolume = 0.2;

const volumeSlider = document.getElementById('volume-slider');
volumeSlider.value = audioVolume;
volumeSlider.addEventListener('input', function() {
    audioVolume = this.value;
});

const initializeAudio = () => {
    if (audioNode != null) {
        audioNode.disconnect();
        audioNode = null;
    }
    
    if (audioContext != null) {
        audioContext.close().then(() => {
            audioContext = null;
            setupAudioContextAndNode();
        });
    } else {
        setupAudioContextAndNode();
    }
    
    function setupAudioContextAndNode() {
        audioContext = new AudioContext();
        audioContext.audioWorklet.addModule('audio.js').then(() => {
            audioNode = new AudioWorkletNode(audioContext, 'gb-audio-processor', {
                processorOptions: { sampleRate: audioContext.sampleRate }
            });
    
            audioNode.port.onmessage = (e) => console.log(e.data);
            audioNode.connect(audioContext.destination);
    
            audioContext.resume();
        });
    }
}

const pushAudioSamples = (audioOutputPtr) => {
    const audioOutput = new Float32Array(
        memory.buffer,
        audioOutputPtr,
        AUDIO_OUTPUT_LEN
    )

    audioNode.port.postMessage(audioOutput.map(sample => sample * audioVolume));
}
// AUDIO------

document.getElementById('file-input').addEventListener('change', function(e) {
    paused = true;
    load_rom(e.target.files[0]);
});

const load_rom = (rom_file) => {
    if (!rom_file) {
        alert("No file selected");
        return;
    }

    stopMainLoop = true;
    initializeAudio();
    clearCanvas()

    var reader = new FileReader();
    reader.readAsArrayBuffer(rom_file);
    reader.onload = function(e) {
        startEmulator(e)
    };
}

const startEmulator = (e) => {
    var arrayBuffer = e.target.result;
    var byteArray = new Uint8Array(arrayBuffer);
    if (window.emulator != null) {
       window.emulator.save_game();
    }

    window.emulator = Emulator.new(byteArray);
    stopMainLoop = false;
    main_loop();
    console.log(canvas.height, canvas.width, window.emulator.game_title());
} 

let stopMainLoop = true;
let paused = false;
let gameSpeed = 0.2;

const speedSlider = document.getElementById('speed-slider');
speedSlider.value = gameSpeed;
speedSlider.addEventListener('input', function() {
    gameSpeed = this.value;
});

let main_loop = () => {
    if (stopMainLoop) {
        return;
    }

    if (!paused) {
        let displayOutputPtr = null;

        let dur = 0;
        while (displayOutputPtr == null) {
            if (dur % 4 == 0) {
                window.emulator.update_joypad(key_status);
            }
            
            window.emulator.step();

            displayOutputPtr = window.emulator.get_display_output();

            const audioOutputPtr = window.emulator.get_audio_output();
            if (audioOutputPtr != null) {
                pushAudioSamples(audioOutputPtr)
            }

            dur++;
        }

        if (displayOutputPtr != null) {
            updateCanvas(displayOutputPtr);
        }
    }

    setTimeout(main_loop, (1000 / 60) * (1 - gameSpeed))
} 

(() => {
    clearCanvas();
})()

document.getElementById("pause-button").addEventListener("click", () => {
    paused = true;
    audioNode.port.postMessage('clearBuffer');
});

document.getElementById("play-button").addEventListener("click", () => {
    paused = false;
});

document.getElementById("restart-button").addEventListener("click", () => {
    load_rom(document.getElementById('file-input').files[0]);
});