import { exportSaveFromDB, importSaveToDB } from "./js/persistence.js";
import { GBEmulator, DEFAULT_GAME_SPEED } from "./js/gbemulator.js";
import { GBAudio, DEFAULT_AUDIO_VOLUME } from "./js/gbaudio.js";
import { GBDisplay } from "./js/gbdisplay.js";

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