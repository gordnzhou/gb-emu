const DB_NAME = "melon-gb";
const STORE_NAME = "saves";
const DB_VERSION = 2;

const SAVE_TYPES = ["ram", "rtc"];

const openSaveDB = (dbMode) => {
    return new Promise((resolve, reject) => {
        const request = indexedDB.open(DB_NAME, DB_VERSION);

        request.onupgradeneeded = (event) => {
            const db = event.target.result;
            if (!db.objectStoreNames.contains(STORE_NAME)) {
                db.createObjectStore(STORE_NAME);
            }
        };

        request.onsuccess = (event) => {
            const db = event.target.result;
            const transaction = db.transaction([STORE_NAME], dbMode);
            const objectStore = transaction.objectStore(STORE_NAME);
            resolve(objectStore);
        }

        request.onerror = function(event) {
            reject(event.target.error);
        };
    });
}

const parseKeyName = (gameId, saveType) => {
    return gameId + ":" + saveType;
}

const readFromSaveDB = async (gameId, saveType) => {
    const objectStore = await openSaveDB("readonly");

    return new Promise((resolve, reject) => {
        const request = objectStore.get(parseKeyName(gameId, saveType));

        request.onsuccess = function(event) {
            if (request.result != null) {
                resolve(request.result);
            } else {
                reject(new Error("No save found for: " + parseKeyName(gameId, saveType)))
            }
        };

        request.onerror = function(event) {
            reject(event.target.error);
        };
    });
}

const writeToSaveDB = async (gameId, saveType, saveData) => {
    const objectStore = await openSaveDB("readwrite");

    return new Promise((resolve, reject) => {
        const request = objectStore.put(saveData, parseKeyName(gameId, saveType));

        request.onerror = function(event) {
            reject(event.target.error);
        };

        resolve(true);
    });
}


window.Persistence = {
    load_from_db: async (gameId, saveType) => {
        try {
            const romSave = await readFromSaveDB(gameId, saveType);
            console.log("loading from: ", parseKeyName(gameId, saveType), romSave);
            window.emulator.load_save(new Uint8Array(romSave), saveType);
        } catch (error) {
            console.log(error);
        }
    },

    save_to_db: async (gameId, saveType, saveData) => {
        try {
            await writeToSaveDB(gameId, saveType, saveData);
            console.log("saving to: ", parseKeyName(gameId, saveType), saveData);
        } catch (error) {
            console.error("Error saving data: ", error);
        }
    },
}

export const exportSaveFromDB = async (gameId) => {
    const saveData = { gameId: gameId };

    let containsSave = false;
    for (const saveType of SAVE_TYPES) {
        try {
            const romSave = await readFromSaveDB(gameId, saveType);
            if (romSave != null) {
                saveData[saveType] = romSave;
                containsSave = true;
            }
        } catch (error) {
            console.log(error);
        }
    }

    if (containsSave) {
        downloadAsSav(saveData, gameId);
    } else {
        alert("No save data is available for the current ROM!");
    }
};

export const importSaveToDB = async (file) => {
    try {
        const saveData = await readSavFile(file);
        const gameId = saveData['gameId'];

        for (const saveType of SAVE_TYPES) {
            if (!saveData.hasOwnProperty(saveType)) {
                continue;
            }

            await writeToSaveDB(gameId, saveType, saveData[saveType]);
        }

        alert("Loaded save data for: " + gameId);
    } catch (error) {
        alert("Could not import save data! " + error);
    }
};

function downloadAsSav(saveData, fileName){
    const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(saveData));
    const downloadAnchorNode = document.createElement('a');
    downloadAnchorNode.setAttribute("href",     dataStr);
    downloadAnchorNode.setAttribute("download", fileName + ".sav");
    document.body.appendChild(downloadAnchorNode);
    downloadAnchorNode.click();
    downloadAnchorNode.remove();
}

function readSavFile(file) {
    return new Promise((resolve, reject) => {
        if (!file.name.endsWith('.sav')) {
            reject(new Error('File must be a .sav file'));
            return;
        }

        const reader = new FileReader();
        reader.onload = () => {
            const saveData = JSON.parse(reader.result);

            if (!isValidSaveData(saveData)) {
                reject(new Error('Invalid save data: ' + saveData));
                return;
            }

            resolve(saveData);
        }

        reader.onerror = reject;
        reader.readAsText(file);
    });
}

//  object must have 'gameId' and 'ram' fields at a MINIMUM
function isValidSaveData(obj) {
    if (typeof obj !== 'object' || obj === null) {
        return false;
    }

    if (!obj.hasOwnProperty('gameId') || !obj.hasOwnProperty('ram')) {
        return false;
    }

    return Array.isArray(obj['ram']) && typeof obj['gameId'] === 'string';
}