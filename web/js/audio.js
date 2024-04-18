class RingBuffer {
    constructor(length) {
        this.buffer = new Float32Array(length);
        this.length = length;
        this.readIndex = 0;
        this.writeIndex = 0;
    }
  
    push(item) {
        this.buffer[this.writeIndex] = item;
        this.writeIndex = (this.writeIndex + 1) % this.length;
        if (this.writeIndex === this.readIndex) {
            this.readIndex = (this.readIndex + 1) % this.length;
        }
    }
  
    pull() {
        const item = this.buffer[this.readIndex];
        this.readIndex = (this.readIndex + 1) % this.length;
        return item;
    }
  
    isEmpty() {
        return this.readIndex === this.writeIndex;
    }
}

class GBAudioProcessor extends AudioWorkletProcessor {
    constructor(options) {
        super();
        this.sampleRate = options.processorOptions.sampleRate;
        this.prev_sample = 0.0;
        this.ringBuffer = new RingBuffer(10 * 4096);
        this.port.postMessage("SAMPLE RATE: " + this.sampleRate);
        this.port.onmessage = event => {
            if (event.data === 'clearBuffer') {
                this.clearBuffer();
            } else {
                event.data.forEach(sample => this.ringBuffer.push(sample));
            }
        };
    }
  
    process(inputs, outputs, parameters) {
        const output = outputs[0];
        // this.port.postMessage("USE: " + output[0].length);

        for (let i = 0; i < output[0].length; ++i) {
            if (this.ringBuffer.isEmpty()) {
                output[0][i] = this.prev_sample;
            } else {
                this.prev_sample = (this.ringBuffer.pull() + this.ringBuffer.pull()) / 2;
                output[0][i] = this.prev_sample;
            }
        }
    
        return true;
    }

    clearBuffer() {
        this.ringBuffer = new RingBuffer(10 * 4096);
    }
}
  
registerProcessor('gb-audio-processor', GBAudioProcessor);