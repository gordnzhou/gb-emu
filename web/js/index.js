import { Emulator } from "gbemulib";
import { memory } from "gbemulib/gbemulib_bg.wasm"
import { exportSaveFromDB, importSaveToDB } from "./persistence";

const DEFAULT_GAME_SPEED = 0.2;
const DEFAULT_AUDIO_VOLUME = 0.2;

const GBInput = (() => {
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

    let keyStatus = 0xFF;

    window.addEventListener('keydown', (event) => {
        for (let i = 0; i < 8; i++) {
            if (event.key == KEYMAPPINGS[i]) {
                keyStatus &= ~(1 << (7 - i));
            }
        }
    });

    window.addEventListener('keyup', (event) => {
        for (let i = 0; i < 8; i++) {
            if (event.key == KEYMAPPINGS[i]) {
                keyStatus |= 1 << (7 - i)
            }
        }
    });

    return {
        getKeyStatus: () => {
            return keyStatus;
        },
    }
})();


const GBDisplay = (() => {
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


const GBAudio = (() => {
    const AUDIO_OUTPUT_LEN = Emulator.audio_output_length();

    let audioContext;
    let audioNode;
    let audioVolume = DEFAULT_AUDIO_VOLUME;

    return {
        initializeAudio: () => {
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
        },

        pushAudioSamples: (audioOutputPtr) => {
            const audioOutput = new Float32Array(
                memory.buffer,
                audioOutputPtr,
                AUDIO_OUTPUT_LEN
            )
        
            audioNode.port.postMessage(audioOutput.map(sample => sample * audioVolume)); 
        },

        clearAudio: () => {
            audioNode.port.postMessage('clearBuffer');
        },

        setAudioVolume: (newAudioVolume) => {
            audioVolume = newAudioVolume;
        }
    }
})();


const GBEmulator = (() => {
    let stopMainLoop = true;
    let paused = false;
    let gameSpeed = DEFAULT_GAME_SPEED;

    const mainLoop = () => {
        if (stopMainLoop) {
            return;
        }
    
        if (!paused) {
            let displayOutputPtr = null;
    
            let dur = 0;
            while (displayOutputPtr == null) {
                if (dur % 2 == 0) {
                    window.emulator.update_joypad(GBInput.getKeyStatus());
                }
                
                window.emulator.step();
    
                displayOutputPtr = window.emulator.get_display_output();
    
                const audioOutputPtr = window.emulator.get_audio_output();
                if (audioOutputPtr != null) {
                    GBAudio.pushAudioSamples(audioOutputPtr)
                }
    
                dur++;
            }
    
            GBDisplay.updateCanvas(displayOutputPtr);
        }
    
        setTimeout(mainLoop, (1000 / 60) * (1 - gameSpeed))
    };
    
    return {
        loadRom: (rom_file) => {
            if (!rom_file) {
                alert("No ROM file selected");
                return;
            }
        
            stopMainLoop = true;
            GBAudio.initializeAudio();
            GBDisplay.clearCanvas()
        
            let reader = new FileReader();
            reader.readAsArrayBuffer(rom_file);
            reader.onload = (e) => {
                if (window.emulator != null) {
                    window.emulator.save_game();
                }

                let arrayBuffer = e.target.result;
                let byteArray = new Uint8Array(arrayBuffer);
                
                try {
                    window.emulator = Emulator.new(byteArray);
                } catch (error) {
                    console.error('Error instantiating Emulator:', error);
                    return;
                }

                stopMainLoop = false;
                mainLoop();
            };
        },

        setPaused: (newPaused) => {
            paused = newPaused;
        },

        setGameSpeed: (newGameSpeed) => {
            gameSpeed = newGameSpeed;
        }
    }
})();

const initializeAutoSave = () => {
    const SAVE_INTERVAL_MS = 10000;
    
    let autoSave = null;
    const autoSaveToggle = document.getElementById("auto-save-toggle");

    const enableAutoSave = () => {
        autoSave = setInterval(() => {
            if (window.emulator != null) {
                window.emulator.save_game();
            }
        }, SAVE_INTERVAL_MS);
        autoSaveToggle.textContent = "Autosave: Enabled";
    }

    const disableAutoSave = () => {
        autoSave = null;
        autoSaveToggle.textContent = "Autosave: Disabled";
    }

    autoSaveToggle.addEventListener("click", () => {
        autoSave == null ? enableAutoSave() : disableAutoSave();
    });

    enableAutoSave();
}

const initializeSpeedSlider = () => {
    const speedSlider = document.getElementById('speed-slider');
    speedSlider.value = DEFAULT_GAME_SPEED;
    speedSlider.addEventListener('input', (e) => GBEmulator.setGameSpeed(e.target.value));
}

const initializeVolumeSlider = () => {
    const volumeSlider = document.getElementById('volume-slider');
    volumeSlider.value = DEFAULT_AUDIO_VOLUME;
    volumeSlider.addEventListener('input', (e) => GBAudio.setAudioVolume(e.target.value));
}

const initializeButtons = () => {
    const fileInput = document.getElementById('file-input');
    fileInput.addEventListener('change', (e) => {
        GBEmulator.setPaused(true);
        GBEmulator.loadRom(e.target.files[0]);
    });
    document.getElementById("file-input-button").addEventListener('click', () => fileInput.click());


    document.getElementById("pause-button").addEventListener("click", () => {
        GBEmulator.setPaused(true);
        GBAudio.clearAudio();
    });
    
    document.getElementById("play-button").addEventListener("click", () => {
        GBEmulator.setPaused(false);
    }); 
    
    document.getElementById("restart-button").addEventListener("click", () => {
        GBAudio.clearAudio();
        GBEmulator.loadRom(fileInput.files[0]);
    });

    document.getElementById("export-save-button").addEventListener("click", () => {
        if (window.emulator != null) {
            exportSaveFromDB(window.emulator.fetch_game_id());
        }
    })

    const importSave = document.getElementById('import-save');
    importSave.addEventListener('change', (e) => {
        importSaveToDB(e.target.files[0])
    });
    document.getElementById('import-save-button').addEventListener('click', () => importSave.click());
}

(() => {
    initializeButtons();
    initializeSpeedSlider();
    initializeVolumeSlider();
    initializeAutoSave();
    GBDisplay.clearCanvas();
})();