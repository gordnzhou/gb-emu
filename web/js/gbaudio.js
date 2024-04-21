import { Emulator } from "gbemulib";
import { memory } from "gbemulib/gbemulib_bg.wasm";

export const DEFAULT_AUDIO_VOLUME = 0.2;

const GB_AUDIO_PATH = "js/audioprocessor.js";
const GB_AUDIO_PROCESSOR = 'gb-audio-processor';

export const GBAudio = (() => {
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
                audioContext.audioWorklet.addModule(GB_AUDIO_PATH).then(() => {
                    audioNode = new AudioWorkletNode(audioContext, GB_AUDIO_PROCESSOR, {
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
            if (audioNode != null) {
                audioNode.port.postMessage('clearBuffer');
            }
        },

        setAudioVolume: (newAudioVolume) => {
            audioVolume = newAudioVolume;
        }
    }
})();