/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "../pkg/gbemulib.js":
/*!**************************!*\
  !*** ../pkg/gbemulib.js ***!
  \**************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   Emulator: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.Emulator),\n/* harmony export */   __wbg_getTime_2bc4375165f02d15: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getTime_2bc4375165f02d15),\n/* harmony export */   __wbg_loadfromdb_768f56715f758797: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_loadfromdb_768f56715f758797),\n/* harmony export */   __wbg_log_70ee89e5e1eef2a1: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_log_70ee89e5e1eef2a1),\n/* harmony export */   __wbg_new0_7d84e5b2cd9fdc73: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new0_7d84e5b2cd9fdc73),\n/* harmony export */   __wbg_new_16b304a2cfa7ff4a: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_16b304a2cfa7ff4a),\n/* harmony export */   __wbg_push_a5b05aedc7234f9f: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_push_a5b05aedc7234f9f),\n/* harmony export */   __wbg_savetodb_88c0a34a5f2fdfc6: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_savetodb_88c0a34a5f2fdfc6),\n/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),\n/* harmony export */   __wbindgen_debug_string: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_debug_string),\n/* harmony export */   __wbindgen_number_new: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_number_new),\n/* harmony export */   __wbindgen_object_drop_ref: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_object_drop_ref),\n/* harmony export */   __wbindgen_throw: () => (/* reexport safe */ _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw)\n/* harmony export */ });\n/* harmony import */ var _gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./gbemulib_bg.wasm */ \"../pkg/gbemulib_bg.wasm\");\n/* harmony import */ var _gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./gbemulib_bg.js */ \"../pkg/gbemulib_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n_gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n(0,_gbemulib_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);\n\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack:///../pkg/gbemulib.js?");

/***/ }),

/***/ "../pkg/gbemulib_bg.js":
/*!*****************************!*\
  !*** ../pkg/gbemulib_bg.js ***!
  \*****************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   Emulator: () => (/* binding */ Emulator),\n/* harmony export */   __wbg_getTime_2bc4375165f02d15: () => (/* binding */ __wbg_getTime_2bc4375165f02d15),\n/* harmony export */   __wbg_loadfromdb_768f56715f758797: () => (/* binding */ __wbg_loadfromdb_768f56715f758797),\n/* harmony export */   __wbg_log_70ee89e5e1eef2a1: () => (/* binding */ __wbg_log_70ee89e5e1eef2a1),\n/* harmony export */   __wbg_new0_7d84e5b2cd9fdc73: () => (/* binding */ __wbg_new0_7d84e5b2cd9fdc73),\n/* harmony export */   __wbg_new_16b304a2cfa7ff4a: () => (/* binding */ __wbg_new_16b304a2cfa7ff4a),\n/* harmony export */   __wbg_push_a5b05aedc7234f9f: () => (/* binding */ __wbg_push_a5b05aedc7234f9f),\n/* harmony export */   __wbg_savetodb_88c0a34a5f2fdfc6: () => (/* binding */ __wbg_savetodb_88c0a34a5f2fdfc6),\n/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),\n/* harmony export */   __wbindgen_debug_string: () => (/* binding */ __wbindgen_debug_string),\n/* harmony export */   __wbindgen_number_new: () => (/* binding */ __wbindgen_number_new),\n/* harmony export */   __wbindgen_object_drop_ref: () => (/* binding */ __wbindgen_object_drop_ref),\n/* harmony export */   __wbindgen_throw: () => (/* binding */ __wbindgen_throw)\n/* harmony export */ });\n/* module decorator */ module = __webpack_require__.hmd(module);\nlet wasm;\nfunction __wbg_set_wasm(val) {\n    wasm = val;\n}\n\n\nconst heap = new Array(128).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 132) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction debugString(val) {\n    // primitive types\n    const type = typeof val;\n    if (type == 'number' || type == 'boolean' || val == null) {\n        return  `${val}`;\n    }\n    if (type == 'string') {\n        return `\"${val}\"`;\n    }\n    if (type == 'symbol') {\n        const description = val.description;\n        if (description == null) {\n            return 'Symbol';\n        } else {\n            return `Symbol(${description})`;\n        }\n    }\n    if (type == 'function') {\n        const name = val.name;\n        if (typeof name == 'string' && name.length > 0) {\n            return `Function(${name})`;\n        } else {\n            return 'Function';\n        }\n    }\n    // objects\n    if (Array.isArray(val)) {\n        const length = val.length;\n        let debug = '[';\n        if (length > 0) {\n            debug += debugString(val[0]);\n        }\n        for(let i = 1; i < length; i++) {\n            debug += ', ' + debugString(val[i]);\n        }\n        debug += ']';\n        return debug;\n    }\n    // Test for built-in\n    const builtInMatches = /\\[object ([^\\]]+)\\]/.exec(toString.call(val));\n    let className;\n    if (builtInMatches.length > 1) {\n        className = builtInMatches[1];\n    } else {\n        // Failed to match the standard '[object ClassName]'\n        return toString.call(val);\n    }\n    if (className == 'Object') {\n        // we're a user defined class or Object\n        // JSON.stringify avoids problems with cycles, and is generally much\n        // easier than looping through ownProperties of `val`.\n        try {\n            return 'Object(' + JSON.stringify(val) + ')';\n        } catch (_) {\n            return 'Object';\n        }\n    }\n    // errors\n    if (val instanceof Error) {\n        return `${val.name}: ${val.message}\\n${val.stack}`;\n    }\n    // TODO we could test for more things here, like `Set`s and `Map`s.\n    return className;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachedUint8Memory0 = null;\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length, 1) >>> 0;\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len, 1) >>> 0;\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n        ptr = realloc(ptr, len, offset, 1) >>> 0;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedInt32Memory0 = null;\n\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);\n    }\n    return cachedInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nfunction passArray8ToWasm0(arg, malloc) {\n    const ptr = malloc(arg.length * 1, 1) >>> 0;\n    getUint8Memory0().set(arg, ptr / 1);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n\nconst EmulatorFinalization = (typeof FinalizationRegistry === 'undefined')\n    ? { register: () => {}, unregister: () => {} }\n    : new FinalizationRegistry(ptr => wasm.__wbg_emulator_free(ptr >>> 0));\n/**\n*/\nclass Emulator {\n\n    static __wrap(ptr) {\n        ptr = ptr >>> 0;\n        const obj = Object.create(Emulator.prototype);\n        obj.__wbg_ptr = ptr;\n        EmulatorFinalization.register(obj, obj.__wbg_ptr, obj);\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.__wbg_ptr;\n        this.__wbg_ptr = 0;\n        EmulatorFinalization.unregister(this);\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        wasm.__wbg_emulator_free(ptr);\n    }\n    /**\n    * @param {Uint8Array} cartridge_bytes\n    * @returns {Emulator}\n    */\n    static new(cartridge_bytes) {\n        const ptr0 = passArray8ToWasm0(cartridge_bytes, wasm.__wbindgen_malloc);\n        const len0 = WASM_VECTOR_LEN;\n        const ret = wasm.emulator_new(ptr0, len0);\n        return Emulator.__wrap(ret);\n    }\n    /**\n    */\n    step() {\n        wasm.emulator_step(this.__wbg_ptr);\n    }\n    /**\n    * @returns {string}\n    */\n    game_title() {\n        let deferred1_0;\n        let deferred1_1;\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            wasm.emulator_game_title(retptr, this.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            deferred1_0 = r0;\n            deferred1_1 = r1;\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);\n        }\n    }\n    /**\n    * @returns {number | undefined}\n    */\n    get_audio_output() {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            wasm.emulator_get_audio_output(retptr, this.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            return r0 === 0 ? undefined : r1 >>> 0;\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    * @returns {number}\n    */\n    static audio_output_length() {\n        const ret = wasm.emulator_audio_output_length();\n        return ret >>> 0;\n    }\n    /**\n    * @returns {number}\n    */\n    static audio_rate() {\n        const ret = wasm.emulator_audio_rate();\n        return ret >>> 0;\n    }\n    /**\n    * @returns {number | undefined}\n    */\n    get_display_output() {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            wasm.emulator_get_display_output(retptr, this.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            return r0 === 0 ? undefined : r1 >>> 0;\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    * @returns {number}\n    */\n    static display_height() {\n        const ret = wasm.emulator_display_height();\n        return ret >>> 0;\n    }\n    /**\n    * @returns {number}\n    */\n    static display_width() {\n        const ret = wasm.emulator_display_width();\n        return ret >>> 0;\n    }\n    /**\n    * @returns {number}\n    */\n    static display_byte_length() {\n        const ret = wasm.emulator_display_byte_length();\n        return ret >>> 0;\n    }\n    /**\n    * @returns {boolean}\n    */\n    entered_hblank() {\n        const ret = wasm.emulator_entered_hblank(this.__wbg_ptr);\n        return ret !== 0;\n    }\n    /**\n    * @param {number} status\n    */\n    update_joypad(status) {\n        wasm.emulator_update_joypad(this.__wbg_ptr, status);\n    }\n    /**\n    */\n    save_game() {\n        wasm.emulator_save_game(this.__wbg_ptr);\n    }\n    /**\n    * @returns {string | undefined}\n    */\n    fetch_game_id() {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            wasm.emulator_fetch_game_id(retptr, this.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            let v1;\n            if (r0 !== 0) {\n                v1 = getStringFromWasm0(r0, r1).slice();\n                wasm.__wbindgen_free(r0, r1 * 1, 1);\n            }\n            return v1;\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    * @param {Uint8Array} data\n    * @param {string} save_type\n    */\n    load_save(data, save_type) {\n        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);\n        const len0 = WASM_VECTOR_LEN;\n        const ptr1 = passStringToWasm0(save_type, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n        const len1 = WASM_VECTOR_LEN;\n        wasm.emulator_load_save(this.__wbg_ptr, ptr0, len0, ptr1, len1);\n    }\n}\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\nfunction __wbg_loadfromdb_768f56715f758797(arg0, arg1, arg2, arg3) {\n    Persistence.load_from_db(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));\n};\n\nfunction __wbg_savetodb_88c0a34a5f2fdfc6(arg0, arg1, arg2, arg3, arg4) {\n    Persistence.save_to_db(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3), takeObject(arg4));\n};\n\nfunction __wbg_log_70ee89e5e1eef2a1(arg0, arg1) {\n    console.log(getStringFromWasm0(arg0, arg1));\n};\n\nfunction __wbindgen_number_new(arg0) {\n    const ret = arg0;\n    return addHeapObject(ret);\n};\n\nfunction __wbg_new_16b304a2cfa7ff4a() {\n    const ret = new Array();\n    return addHeapObject(ret);\n};\n\nfunction __wbg_push_a5b05aedc7234f9f(arg0, arg1) {\n    const ret = getObject(arg0).push(getObject(arg1));\n    return ret;\n};\n\nfunction __wbg_getTime_2bc4375165f02d15(arg0) {\n    const ret = getObject(arg0).getTime();\n    return ret;\n};\n\nfunction __wbg_new0_7d84e5b2cd9fdc73() {\n    const ret = new Date();\n    return addHeapObject(ret);\n};\n\nfunction __wbindgen_debug_string(arg0, arg1) {\n    const ret = debugString(getObject(arg1));\n    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n    const len1 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len1;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr1;\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n\n//# sourceURL=webpack:///../pkg/gbemulib_bg.js?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _js_persistence_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./js/persistence.js */ \"./js/persistence.js\");\n/* harmony import */ var _js_gbemulator_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./js/gbemulator.js */ \"./js/gbemulator.js\");\n/* harmony import */ var _js_gbaudio_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./js/gbaudio.js */ \"./js/gbaudio.js\");\n/* harmony import */ var _js_gbdisplay_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./js/gbdisplay.js */ \"./js/gbdisplay.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_js_gbemulator_js__WEBPACK_IMPORTED_MODULE_1__, _js_gbaudio_js__WEBPACK_IMPORTED_MODULE_2__, _js_gbdisplay_js__WEBPACK_IMPORTED_MODULE_3__]);\n([_js_gbemulator_js__WEBPACK_IMPORTED_MODULE_1__, _js_gbaudio_js__WEBPACK_IMPORTED_MODULE_2__, _js_gbdisplay_js__WEBPACK_IMPORTED_MODULE_3__] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__);\n\n\n\n\n\nconst initializeAutoSave = () => {\n    const SAVE_INTERVAL_MS = 10000;\n    \n    let autoSave = null;\n    const autoSaveToggle = document.getElementById(\"auto-save-toggle\");\n\n    const enableAutoSave = () => {\n        autoSave = setInterval(() => {\n            if (window.emulator != null) {\n                window.emulator.save_game();\n            }\n        }, SAVE_INTERVAL_MS);\n        autoSaveToggle.textContent = \"Autosave: Enabled\";\n    }\n\n    const disableAutoSave = () => {\n        autoSave = null;\n        autoSaveToggle.textContent = \"Autosave: Disabled\";\n    }\n\n    autoSaveToggle.addEventListener(\"click\", () => {\n        autoSave == null ? enableAutoSave() : disableAutoSave();\n    });\n\n    enableAutoSave();\n}\n\nconst initializeSpeedSlider = () => {\n    const speedSlider = document.getElementById('speed-slider');\n    speedSlider.value = _js_gbemulator_js__WEBPACK_IMPORTED_MODULE_1__.DEFAULT_GAME_SPEED;\n    speedSlider.addEventListener('input', (e) => _js_gbemulator_js__WEBPACK_IMPORTED_MODULE_1__.GBEmulator.setGameSpeed(e.target.value));\n}\n\nconst initializeVolumeSlider = () => {\n    const volumeSlider = document.getElementById('volume-slider');\n    volumeSlider.value = _js_gbaudio_js__WEBPACK_IMPORTED_MODULE_2__.DEFAULT_AUDIO_VOLUME;\n    volumeSlider.addEventListener('input', (e) => _js_gbaudio_js__WEBPACK_IMPORTED_MODULE_2__.GBAudio.setAudioVolume(e.target.value));\n}\n\nconst initializeButtons = () => {\n    const fileInput = document.getElementById('file-input');\n    fileInput.addEventListener('change', (e) => {\n        _js_gbemulator_js__WEBPACK_IMPORTED_MODULE_1__.GBEmulator.setPaused(true);\n        _js_gbemulator_js__WEBPACK_IMPORTED_MODULE_1__.GBEmulator.loadRom(e.target.files[0]);\n    });\n    document.getElementById(\"file-input-button\").addEventListener('click', () => fileInput.click());\n\n\n    document.getElementById(\"pause-button\").addEventListener(\"click\", () => {\n        _js_gbemulator_js__WEBPACK_IMPORTED_MODULE_1__.GBEmulator.setPaused(true);\n        _js_gbaudio_js__WEBPACK_IMPORTED_MODULE_2__.GBAudio.clearAudio();\n    });\n    \n    document.getElementById(\"play-button\").addEventListener(\"click\", () => {\n        _js_gbemulator_js__WEBPACK_IMPORTED_MODULE_1__.GBEmulator.setPaused(false);\n    }); \n    \n    document.getElementById(\"restart-button\").addEventListener(\"click\", () => {\n        _js_gbemulator_js__WEBPACK_IMPORTED_MODULE_1__.GBEmulator.loadRom(fileInput.files[0]);\n    });\n\n    document.getElementById(\"export-save-button\").addEventListener(\"click\", () => {\n        if (window.emulator != null) {\n            (0,_js_persistence_js__WEBPACK_IMPORTED_MODULE_0__.exportSaveFromDB)(window.emulator.fetch_game_id());\n        }\n    })\n\n    const importSave = document.getElementById('import-save');\n    importSave.addEventListener('change', (e) => {\n        (0,_js_persistence_js__WEBPACK_IMPORTED_MODULE_0__.importSaveToDB)(e.target.files[0])\n    });\n    document.getElementById('import-save-button').addEventListener('click', () => importSave.click());\n}\n\n(() => {\n    initializeButtons();\n    initializeSpeedSlider();\n    initializeVolumeSlider();\n    initializeAutoSave();\n    _js_gbdisplay_js__WEBPACK_IMPORTED_MODULE_3__.GBDisplay.clearCanvas();\n})();\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./js/gbaudio.js":
/*!***********************!*\
  !*** ./js/gbaudio.js ***!
  \***********************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   DEFAULT_AUDIO_VOLUME: () => (/* binding */ DEFAULT_AUDIO_VOLUME),\n/* harmony export */   GBAudio: () => (/* binding */ GBAudio)\n/* harmony export */ });\n/* harmony import */ var gbemulib__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! gbemulib */ \"../pkg/gbemulib.js\");\n/* harmony import */ var gbemulib_gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! gbemulib/gbemulib_bg.wasm */ \"../pkg/gbemulib_bg.wasm\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([gbemulib__WEBPACK_IMPORTED_MODULE_0__, gbemulib_gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n([gbemulib__WEBPACK_IMPORTED_MODULE_0__, gbemulib_gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__);\n\n\n\nconst DEFAULT_AUDIO_VOLUME = 0.2;\n\nconst GB_AUDIO_PATH = \"js/audioprocessor.js\";\nconst GB_AUDIO_PROCESSOR = 'gb-audio-processor';\n\nconst GBAudio = (() => {\n    const AUDIO_OUTPUT_LEN = gbemulib__WEBPACK_IMPORTED_MODULE_0__.Emulator.audio_output_length();\n\n    let audioContext;\n    let audioNode;\n    let audioVolume = DEFAULT_AUDIO_VOLUME;\n\n    return {\n        initializeAudio: () => {\n            if (audioNode != null) {\n                audioNode.disconnect();\n                audioNode = null;\n            }\n            \n            if (audioContext != null) {\n                audioContext.close().then(() => {\n                    audioContext = null;\n                    setupAudioContextAndNode();\n                });\n            } else {\n                setupAudioContextAndNode();\n            }\n            \n            function setupAudioContextAndNode() {\n                audioContext = new AudioContext();\n                audioContext.audioWorklet.addModule(GB_AUDIO_PATH).then(() => {\n                    audioNode = new AudioWorkletNode(audioContext, GB_AUDIO_PROCESSOR, {\n                        processorOptions: { sampleRate: audioContext.sampleRate }\n                    });\n            \n                    audioNode.port.onmessage = (e) => console.log(e.data);\n                    audioNode.connect(audioContext.destination);\n            \n                    audioContext.resume();\n                });\n            }\n        },\n\n        pushAudioSamples: (audioOutputPtr) => {\n            const audioOutput = new Float32Array(\n                gbemulib_gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.memory.buffer,\n                audioOutputPtr,\n                AUDIO_OUTPUT_LEN\n            )\n            \n            audioNode.port.postMessage(audioOutput.map(sample => sample * audioVolume)); \n        },\n\n        clearAudio: () => {\n            if (audioNode != null) {\n                audioNode.port.postMessage('clearBuffer');\n            }\n        },\n\n        setAudioVolume: (newAudioVolume) => {\n            audioVolume = newAudioVolume;\n        }\n    }\n})();\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack:///./js/gbaudio.js?");

/***/ }),

/***/ "./js/gbdisplay.js":
/*!*************************!*\
  !*** ./js/gbdisplay.js ***!
  \*************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   GBDisplay: () => (/* binding */ GBDisplay)\n/* harmony export */ });\n/* harmony import */ var gbemulib__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! gbemulib */ \"../pkg/gbemulib.js\");\n/* harmony import */ var gbemulib_gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! gbemulib/gbemulib_bg.wasm */ \"../pkg/gbemulib_bg.wasm\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([gbemulib__WEBPACK_IMPORTED_MODULE_0__, gbemulib_gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n([gbemulib__WEBPACK_IMPORTED_MODULE_0__, gbemulib_gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__);\n\n\n\nconst GBDisplay = (() => {\n    const WIDTH = gbemulib__WEBPACK_IMPORTED_MODULE_0__.Emulator.display_width();\n    const HEIGHT = gbemulib__WEBPACK_IMPORTED_MODULE_0__.Emulator.display_height();\n    const DISPLAY_BYTE_LEN = gbemulib__WEBPACK_IMPORTED_MODULE_0__.Emulator.display_byte_length();\n\n    const CANVAS_SCALE = 3;\n    const CLEAR_COLOUR = \"#FFFFE8\";\n\n    const canvas = document.getElementById(\"gb-display\");\n\n    canvas.height = HEIGHT * CANVAS_SCALE;\n    canvas.width = WIDTH * CANVAS_SCALE;\n\n    const ctx = canvas.getContext('2d');\n\n    ctx.imageSmoothingEnabled = false;\n    ctx.scale(CANVAS_SCALE, CANVAS_SCALE);\n\n    return {\n        updateCanvas: (displayOutputPtr) => {\n            const display_output = new Uint8Array(\n                gbemulib_gbemulib_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.memory.buffer, \n                displayOutputPtr, \n                WIDTH * HEIGHT * DISPLAY_BYTE_LEN\n            );\n            \n            let tempCanvas = document.createElement('canvas');\n            tempCanvas.width = WIDTH;\n            tempCanvas.height = HEIGHT;\n            let tempCtx = tempCanvas.getContext('2d');\n            let imageData = ctx.createImageData(WIDTH, HEIGHT);\n            let data = imageData.data;\n    \n            let i = 0;\n            for (let y = 0; y < HEIGHT; y++) {\n                for (let x = 0; x < WIDTH; x++) {\n                    let index = (y * WIDTH + x) * DISPLAY_BYTE_LEN\n                    data[i++] = display_output[index + 2];\n                    data[i++] = display_output[index + 1];\n                    data[i++] = display_output[index];\n                    data[i++] = display_output[index + 3];\n                }\n            }\n    \n            tempCtx.putImageData(imageData, 0, 0);\n            ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);\n            ctx.drawImage(tempCanvas, 0, 0);\n        },\n\n        clearCanvas: () => {   \n            ctx.fillStyle = CLEAR_COLOUR;\n            ctx.fillRect(0, 0, canvas.width, canvas.height);\n        }\n    }\n})();\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack:///./js/gbdisplay.js?");

/***/ }),

/***/ "./js/gbemulator.js":
/*!**************************!*\
  !*** ./js/gbemulator.js ***!
  \**************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   DEFAULT_GAME_SPEED: () => (/* binding */ DEFAULT_GAME_SPEED),\n/* harmony export */   GBEmulator: () => (/* binding */ GBEmulator)\n/* harmony export */ });\n/* harmony import */ var gbemulib__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! gbemulib */ \"../pkg/gbemulib.js\");\n/* harmony import */ var _gbinput_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./gbinput.js */ \"./js/gbinput.js\");\n/* harmony import */ var _gbdisplay_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./gbdisplay.js */ \"./js/gbdisplay.js\");\n/* harmony import */ var _gbaudio_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./gbaudio.js */ \"./js/gbaudio.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([gbemulib__WEBPACK_IMPORTED_MODULE_0__, _gbdisplay_js__WEBPACK_IMPORTED_MODULE_2__, _gbaudio_js__WEBPACK_IMPORTED_MODULE_3__]);\n([gbemulib__WEBPACK_IMPORTED_MODULE_0__, _gbdisplay_js__WEBPACK_IMPORTED_MODULE_2__, _gbaudio_js__WEBPACK_IMPORTED_MODULE_3__] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__);\n\n\n\n\n\nconst DEFAULT_GAME_SPEED = 0.3;\n\nconst GBEmulator = (() => {\n    let stopMainLoop = true;\n    let paused = false;\n    let gameSpeed = DEFAULT_GAME_SPEED;\n\n    const mainLoop = () => {\n        if (stopMainLoop) {\n            return;\n        }\n    \n        if (!paused) {\n            let displayOutputPtr = null;\n    \n            let dur = 0;\n            while (displayOutputPtr == null) {\n                if (dur % 2 == 0) {\n                    window.emulator.update_joypad(_gbinput_js__WEBPACK_IMPORTED_MODULE_1__.GBInput.getKeyStatus());\n                }\n                \n                window.emulator.step();\n    \n                displayOutputPtr = window.emulator.get_display_output();\n    \n                const audioOutputPtr = window.emulator.get_audio_output();\n                if (audioOutputPtr != null) {\n                    _gbaudio_js__WEBPACK_IMPORTED_MODULE_3__.GBAudio.pushAudioSamples(audioOutputPtr)\n                }\n    \n                dur++;\n            }\n    \n            _gbdisplay_js__WEBPACK_IMPORTED_MODULE_2__.GBDisplay.updateCanvas(displayOutputPtr);\n        }\n    \n        setTimeout(mainLoop, (1000 / 60) * (1 - gameSpeed))\n    };\n    \n    return {\n        loadRom: (rom_file) => {\n            if (!rom_file) {\n                alert(\"No ROM file selected\");\n                return;\n            }\n        \n            stopMainLoop = true;\n            _gbaudio_js__WEBPACK_IMPORTED_MODULE_3__.GBAudio.initializeAudio();\n            _gbaudio_js__WEBPACK_IMPORTED_MODULE_3__.GBAudio.clearAudio();\n            _gbdisplay_js__WEBPACK_IMPORTED_MODULE_2__.GBDisplay.clearCanvas();\n        \n            let reader = new FileReader();\n            reader.readAsArrayBuffer(rom_file);\n            reader.onload = (e) => {\n                if (window.emulator != null) {\n                    window.emulator.save_game();\n                }\n\n                let arrayBuffer = e.target.result;\n                let byteArray = new Uint8Array(arrayBuffer);\n                \n                try {\n                    window.emulator = gbemulib__WEBPACK_IMPORTED_MODULE_0__.Emulator.new(byteArray);\n                } catch (error) {\n                    console.error('Error instantiating Emulator:', error);\n                    alert(\"Unable to load ROM file :(\")\n                    return;\n                }\n\n                stopMainLoop = false;\n                mainLoop();\n            };\n        },\n\n        setPaused: (newPaused) => {\n            paused = newPaused;\n        },\n\n        setGameSpeed: (newGameSpeed) => {\n            gameSpeed = newGameSpeed;\n        }\n    }\n})();\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack:///./js/gbemulator.js?");

/***/ }),

/***/ "./js/gbinput.js":
/*!***********************!*\
  !*** ./js/gbinput.js ***!
  \***********************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   GBInput: () => (/* binding */ GBInput)\n/* harmony export */ });\nconst GBInput = (() => {\n    // in order of: START, SELECT, B, A, DOWN, UP, LEFT, RIGHT.\n    const KEYMAPPINGS = [\n        'Enter',\n        'ShiftRight',\n        'z',\n        'x',\n        'ArrowDown',\n        'ArrowUp',\n        'ArrowLeft',\n        'ArrowRight',\n    ];\n\n    let keyStatus = 0xFF;\n\n    window.addEventListener('keydown', (event) => {\n        for (let i = 0; i < 8; i++) {\n            if (event.key == KEYMAPPINGS[i]) {\n                keyStatus &= ~(1 << (7 - i));\n            }\n        }\n    });\n\n    window.addEventListener('keyup', (event) => {\n        for (let i = 0; i < 8; i++) {\n            if (event.key == KEYMAPPINGS[i]) {\n                keyStatus |= 1 << (7 - i)\n            }\n        }\n    });\n\n    return {\n        getKeyStatus: () => {\n            return keyStatus;\n        },\n    }\n})();\n\n//# sourceURL=webpack:///./js/gbinput.js?");

/***/ }),

/***/ "./js/persistence.js":
/*!***************************!*\
  !*** ./js/persistence.js ***!
  \***************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   exportSaveFromDB: () => (/* binding */ exportSaveFromDB),\n/* harmony export */   importSaveToDB: () => (/* binding */ importSaveToDB)\n/* harmony export */ });\nconst DB_NAME = \"melon-gb\";\nconst STORE_NAME = \"saves\";\nconst DB_VERSION = 2;\n\nconst SAVE_TYPES = [\"ram\", \"rtc\"];\n\nconst openSaveDB = (dbMode) => {\n    return new Promise((resolve, reject) => {\n        const request = indexedDB.open(DB_NAME, DB_VERSION);\n\n        request.onupgradeneeded = (event) => {\n            const db = event.target.result;\n            if (!db.objectStoreNames.contains(STORE_NAME)) {\n                db.createObjectStore(STORE_NAME);\n            }\n        };\n\n        request.onsuccess = (event) => {\n            const db = event.target.result;\n            const transaction = db.transaction([STORE_NAME], dbMode);\n            const objectStore = transaction.objectStore(STORE_NAME);\n            resolve(objectStore);\n        }\n\n        request.onerror = function(event) {\n            reject(event.target.error);\n        };\n    });\n}\n\nconst parseKeyName = (gameId, saveType) => {\n    return gameId + \":\" + saveType;\n}\n\nconst readFromSaveDB = async (gameId, saveType) => {\n    const objectStore = await openSaveDB(\"readonly\");\n\n    return new Promise((resolve, reject) => {\n        const request = objectStore.get(parseKeyName(gameId, saveType));\n\n        request.onsuccess = function(event) {\n            if (request.result != null) {\n                resolve(request.result);\n            } else {\n                reject(new Error(\"No save found for: \" + parseKeyName(gameId, saveType)))\n            }\n        };\n\n        request.onerror = function(event) {\n            reject(event.target.error);\n        };\n    });\n}\n\nconst writeToSaveDB = async (gameId, saveType, saveData) => {\n    const objectStore = await openSaveDB(\"readwrite\");\n\n    return new Promise((resolve, reject) => {\n        const request = objectStore.put(saveData, parseKeyName(gameId, saveType));\n\n        request.onerror = function(event) {\n            reject(event.target.error);\n        };\n\n        resolve(true);\n    });\n}\n\n\nwindow.Persistence = {\n    load_from_db: async (gameId, saveType) => {\n        try {\n            const romSave = await readFromSaveDB(gameId, saveType);\n            console.log(\"loading from: \", parseKeyName(gameId, saveType), romSave);\n            window.emulator.load_save(new Uint8Array(romSave), saveType);\n        } catch (error) {\n            console.log(error);\n        }\n    },\n\n    save_to_db: async (gameId, saveType, saveData) => {\n        try {\n            await writeToSaveDB(gameId, saveType, saveData);\n            console.log(\"saving to: \", parseKeyName(gameId, saveType), saveData);\n        } catch (error) {\n            console.error(\"Error saving data: \", error);\n        }\n    },\n}\n\nconst exportSaveFromDB = async (gameId) => {\n    const saveData = { gameId: gameId };\n\n    let containsSave = false;\n    for (const saveType of SAVE_TYPES) {\n        try {\n            const romSave = await readFromSaveDB(gameId, saveType);\n            if (romSave != null) {\n                saveData[saveType] = romSave;\n                containsSave = true;\n            }\n        } catch (error) {\n            console.log(error);\n        }\n    }\n\n    if (containsSave) {\n        downloadAsSav(saveData, gameId);\n    } else {\n        alert(\"No save data is available for the current ROM!\");\n    }\n};\n\nconst importSaveToDB = async (file) => {\n    try {\n        const saveData = await readSavFile(file);\n        const gameId = saveData['gameId'];\n\n        for (const saveType of SAVE_TYPES) {\n            if (!saveData.hasOwnProperty(saveType)) {\n                continue;\n            }\n\n            await writeToSaveDB(gameId, saveType, saveData[saveType]);\n        }\n\n        alert(\"Loaded save data for: \" + gameId);\n    } catch (error) {\n        alert(\"Could not import save data! \" + error);\n    }\n};\n\nfunction downloadAsSav(saveData, fileName){\n    const dataStr = \"data:text/json;charset=utf-8,\" + encodeURIComponent(JSON.stringify(saveData));\n    const downloadAnchorNode = document.createElement('a');\n    downloadAnchorNode.setAttribute(\"href\",     dataStr);\n    downloadAnchorNode.setAttribute(\"download\", fileName + \".sav\");\n    document.body.appendChild(downloadAnchorNode);\n    downloadAnchorNode.click();\n    downloadAnchorNode.remove();\n}\n\nfunction readSavFile(file) {\n    return new Promise((resolve, reject) => {\n        if (!file.name.endsWith('.sav')) {\n            reject(new Error('File must be a .sav file'));\n            return;\n        }\n\n        const reader = new FileReader();\n        reader.onload = () => {\n            const saveData = JSON.parse(reader.result);\n\n            if (!isValidSaveData(saveData)) {\n                reject(new Error('Invalid save data: ' + saveData));\n                return;\n            }\n\n            resolve(saveData);\n        }\n\n        reader.onerror = reject;\n        reader.readAsText(file);\n    });\n}\n\n//  object must have 'gameId' and 'ram' fields at a MINIMUM\nfunction isValidSaveData(obj) {\n    if (typeof obj !== 'object' || obj === null) {\n        return false;\n    }\n\n    if (!obj.hasOwnProperty('gameId') || !obj.hasOwnProperty('ram')) {\n        return false;\n    }\n\n    return Array.isArray(obj['ram']) && typeof obj['gameId'] === 'string';\n}\n\n//# sourceURL=webpack:///./js/persistence.js?");

/***/ }),

/***/ "../pkg/gbemulib_bg.wasm":
/*!*******************************!*\
  !*** ../pkg/gbemulib_bg.wasm ***!
  \*******************************/
/***/ ((module, exports, __webpack_require__) => {

eval("/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./gbemulib_bg.js */ \"../pkg/gbemulib_bg.js\");\nmodule.exports = __webpack_require__.v(exports, module.id, \"629e071f39800fdec23b\", {\n\t\"./gbemulib_bg.js\": {\n\t\t\"__wbindgen_object_drop_ref\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_drop_ref,\n\t\t\"__wbg_loadfromdb_768f56715f758797\": WEBPACK_IMPORTED_MODULE_0.__wbg_loadfromdb_768f56715f758797,\n\t\t\"__wbg_savetodb_88c0a34a5f2fdfc6\": WEBPACK_IMPORTED_MODULE_0.__wbg_savetodb_88c0a34a5f2fdfc6,\n\t\t\"__wbg_log_70ee89e5e1eef2a1\": WEBPACK_IMPORTED_MODULE_0.__wbg_log_70ee89e5e1eef2a1,\n\t\t\"__wbindgen_number_new\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_number_new,\n\t\t\"__wbg_new_16b304a2cfa7ff4a\": WEBPACK_IMPORTED_MODULE_0.__wbg_new_16b304a2cfa7ff4a,\n\t\t\"__wbg_push_a5b05aedc7234f9f\": WEBPACK_IMPORTED_MODULE_0.__wbg_push_a5b05aedc7234f9f,\n\t\t\"__wbg_getTime_2bc4375165f02d15\": WEBPACK_IMPORTED_MODULE_0.__wbg_getTime_2bc4375165f02d15,\n\t\t\"__wbg_new0_7d84e5b2cd9fdc73\": WEBPACK_IMPORTED_MODULE_0.__wbg_new0_7d84e5b2cd9fdc73,\n\t\t\"__wbindgen_debug_string\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_debug_string,\n\t\t\"__wbindgen_throw\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw\n\t}\n});\n\n//# sourceURL=webpack:///../pkg/gbemulib_bg.wasm?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			id: moduleId,
/******/ 			loaded: false,
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Flag the module as loaded
/******/ 		module.loaded = true;
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/async module */
/******/ 	(() => {
/******/ 		var webpackQueues = typeof Symbol === "function" ? Symbol("webpack queues") : "__webpack_queues__";
/******/ 		var webpackExports = typeof Symbol === "function" ? Symbol("webpack exports") : "__webpack_exports__";
/******/ 		var webpackError = typeof Symbol === "function" ? Symbol("webpack error") : "__webpack_error__";
/******/ 		var resolveQueue = (queue) => {
/******/ 			if(queue && queue.d < 1) {
/******/ 				queue.d = 1;
/******/ 				queue.forEach((fn) => (fn.r--));
/******/ 				queue.forEach((fn) => (fn.r-- ? fn.r++ : fn()));
/******/ 			}
/******/ 		}
/******/ 		var wrapDeps = (deps) => (deps.map((dep) => {
/******/ 			if(dep !== null && typeof dep === "object") {
/******/ 				if(dep[webpackQueues]) return dep;
/******/ 				if(dep.then) {
/******/ 					var queue = [];
/******/ 					queue.d = 0;
/******/ 					dep.then((r) => {
/******/ 						obj[webpackExports] = r;
/******/ 						resolveQueue(queue);
/******/ 					}, (e) => {
/******/ 						obj[webpackError] = e;
/******/ 						resolveQueue(queue);
/******/ 					});
/******/ 					var obj = {};
/******/ 					obj[webpackQueues] = (fn) => (fn(queue));
/******/ 					return obj;
/******/ 				}
/******/ 			}
/******/ 			var ret = {};
/******/ 			ret[webpackQueues] = x => {};
/******/ 			ret[webpackExports] = dep;
/******/ 			return ret;
/******/ 		}));
/******/ 		__webpack_require__.a = (module, body, hasAwait) => {
/******/ 			var queue;
/******/ 			hasAwait && ((queue = []).d = -1);
/******/ 			var depQueues = new Set();
/******/ 			var exports = module.exports;
/******/ 			var currentDeps;
/******/ 			var outerResolve;
/******/ 			var reject;
/******/ 			var promise = new Promise((resolve, rej) => {
/******/ 				reject = rej;
/******/ 				outerResolve = resolve;
/******/ 			});
/******/ 			promise[webpackExports] = exports;
/******/ 			promise[webpackQueues] = (fn) => (queue && fn(queue), depQueues.forEach(fn), promise["catch"](x => {}));
/******/ 			module.exports = promise;
/******/ 			body((deps) => {
/******/ 				currentDeps = wrapDeps(deps);
/******/ 				var fn;
/******/ 				var getResult = () => (currentDeps.map((d) => {
/******/ 					if(d[webpackError]) throw d[webpackError];
/******/ 					return d[webpackExports];
/******/ 				}))
/******/ 				var promise = new Promise((resolve) => {
/******/ 					fn = () => (resolve(getResult));
/******/ 					fn.r = 0;
/******/ 					var fnQueue = (q) => (q !== queue && !depQueues.has(q) && (depQueues.add(q), q && !q.d && (fn.r++, q.push(fn))));
/******/ 					currentDeps.map((dep) => (dep[webpackQueues](fnQueue)));
/******/ 				});
/******/ 				return fn.r ? promise : getResult();
/******/ 			}, (err) => ((err ? reject(promise[webpackError] = err) : outerResolve(exports)), resolveQueue(queue)));
/******/ 			queue && queue.d < 0 && (queue.d = 0);
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/global */
/******/ 	(() => {
/******/ 		__webpack_require__.g = (function() {
/******/ 			if (typeof globalThis === 'object') return globalThis;
/******/ 			try {
/******/ 				return this || new Function('return this')();
/******/ 			} catch (e) {
/******/ 				if (typeof window === 'object') return window;
/******/ 			}
/******/ 		})();
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/harmony module decorator */
/******/ 	(() => {
/******/ 		__webpack_require__.hmd = (module) => {
/******/ 			module = Object.create(module);
/******/ 			if (!module.children) module.children = [];
/******/ 			Object.defineProperty(module, 'exports', {
/******/ 				enumerable: true,
/******/ 				set: () => {
/******/ 					throw new Error('ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: ' + module.id);
/******/ 				}
/******/ 			});
/******/ 			return module;
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/wasm loading */
/******/ 	(() => {
/******/ 		__webpack_require__.v = (exports, wasmModuleId, wasmModuleHash, importsObj) => {
/******/ 			var req = fetch(__webpack_require__.p + "" + wasmModuleHash + ".module.wasm");
/******/ 			var fallback = () => (req
/******/ 				.then((x) => (x.arrayBuffer()))
/******/ 				.then((bytes) => (WebAssembly.instantiate(bytes, importsObj)))
/******/ 				.then((res) => (Object.assign(exports, res.instance.exports))));
/******/ 			return req.then((res) => {
/******/ 				if (typeof WebAssembly.instantiateStreaming === "function") {
/******/ 					return WebAssembly.instantiateStreaming(res, importsObj)
/******/ 						.then(
/******/ 							(res) => (Object.assign(exports, res.instance.exports)),
/******/ 							(e) => {
/******/ 								if(res.headers.get("Content-Type") !== "application/wasm") {
/******/ 									console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
/******/ 									return fallback();
/******/ 								}
/******/ 								throw e;
/******/ 							}
/******/ 						);
/******/ 				}
/******/ 				return fallback();
/******/ 			});
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/publicPath */
/******/ 	(() => {
/******/ 		var scriptUrl;
/******/ 		if (__webpack_require__.g.importScripts) scriptUrl = __webpack_require__.g.location + "";
/******/ 		var document = __webpack_require__.g.document;
/******/ 		if (!scriptUrl && document) {
/******/ 			if (document.currentScript)
/******/ 				scriptUrl = document.currentScript.src;
/******/ 			if (!scriptUrl) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				if(scripts.length) {
/******/ 					var i = scripts.length - 1;
/******/ 					while (i > -1 && (!scriptUrl || !/^http(s?):/.test(scriptUrl))) scriptUrl = scripts[i--].src;
/******/ 				}
/******/ 			}
/******/ 		}
/******/ 		// When supporting browsers where an automatic publicPath is not supported you must specify an output.publicPath manually via configuration
/******/ 		// or pass an empty string ("") and set the __webpack_public_path__ variable from your code to use your own logic.
/******/ 		if (!scriptUrl) throw new Error("Automatic publicPath is not supported in this browser");
/******/ 		scriptUrl = scriptUrl.replace(/#.*$/, "").replace(/\?.*$/, "").replace(/\/[^\/]+$/, "/");
/******/ 		__webpack_require__.p = scriptUrl;
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./index.js");
/******/ 	
/******/ })()
;