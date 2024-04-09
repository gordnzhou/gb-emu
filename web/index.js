import { Emulator } from "gbemulib";


let emulator = null;

document.getElementById('fileInput').addEventListener('change', function(e) {
    var file = e.target.files[0];
    if (!file) {
        console.log("No file selected");
        return;
    }

    var reader = new FileReader();
    reader.onload = function(e) {
        var arrayBuffer = e.target.result;
        var byteArray = new Uint8Array(arrayBuffer);

        console.log(byteArray)

        emulator = Emulator.new(byteArray);
    
        while (!emulator.entered_hblank()) {
            emulator.step();
        }
        console.log(emulator.entered_hblank(), "DHjdsfj");
    };
    reader.readAsArrayBuffer(file);
});
