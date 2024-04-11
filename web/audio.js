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
    constructor() {
        super();
        this.ringBuffer = new RingBuffer(8 * 2048);
        this.port.onmessage = event => {
            this.ringBuffer.push(event.data);
        };
    }
  
    process(inputs, outputs, parameters) {
        const output = outputs[0];
        
        for (let i = 0; i < output[channel].length; ++i) {
            output[0][i] = this.ringBuffer.pull();
            output[1][i] = this.ringBuffer.pull();
        }

        return true;
    }
}
  
registerProcessor('gb-audio-processor', GBAudioProcessor);