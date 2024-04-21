import { Emulator } from "gbemulib";
import { GBInput } from "./gbinput.js";
import { GBDisplay } from "./gbdisplay.js";
import { GBAudio } from "./gbaudio.js";

export const DEFAULT_GAME_SPEED = 0.3;

export const GBEmulator = (() => {
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
            GBAudio.clearAudio();
            GBDisplay.clearCanvas();
        
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
                    alert("Unable to load ROM file :(")
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