const DB_NAME = "melon-gb";
const STORE_NAME = "saves";

window.Persistence = {
    load_from_db: function(gameId, saveType) {
        const request = indexedDB.open(DB_NAME, 2);

        request.onupgradeneeded = function(event) {
            const db = event.target.result;
            if (!db.objectStoreNames.contains(STORE_NAME)) {
                db.createObjectStore(STORE_NAME);
            }
        };

        request.onsuccess = function(event) {
            const db = event.target.result;

            const transaction = db.transaction([STORE_NAME], "readonly");
            const objectStore = transaction.objectStore(STORE_NAME);

            const keyName = gameId + ":" + saveType;
            const request = objectStore.get(keyName);

            request.onsuccess = function(event) {
                result = request.result ? request.result : null;
                if (result != null) {
                    console.log("loading from: ", keyName, result);
                    window.emulator.load_save(new Uint8Array(result), saveType);
                }
            };

            request.onerror = function(event) {
                console.log("Error loading data", event.target.error);
            };
        };

        request.onerror = function(event) {
            console.log("Error opening database", event.target.error);
        };
    },

    save_to_db: function(gameId, saveType, data) {
        const request = indexedDB.open(DB_NAME, 2);

        request.onupgradeneeded = function(event) {
            const db = event.target.result;
            if (!db.objectStoreNames.contains(STORE_NAME)) {
                db.createObjectStore(STORE_NAME);
            }
        };

        request.onsuccess = function(event) {
            const db = event.target.result;

            const transaction = db.transaction([STORE_NAME], "readwrite");
            const objectStore = transaction.objectStore(STORE_NAME);

            const keyName = gameId + ":" + saveType;
            console.log("saving to: ", keyName, data)
            const request = objectStore.put(data, keyName);

            request.onerror = function(event) {
                console.log("Error saving data", event.target.error);
            };
        };

        request.onerror = function(event) {
            console.log("Error opening database", event.target.error);
        };
    }
}