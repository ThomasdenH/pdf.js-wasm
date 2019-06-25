let wasm;
const { TextDecoder } = require(String.raw`util`);

/**
* @param {number} level
* @returns {void}
*/
module.exports.setVerbosityLevel = function(level) {
    return wasm.setVerbosityLevel(level);
};

/**
* @returns {number}
*/
module.exports.getVerbosityLevel = function() {
    return wasm.getVerbosityLevel();
};

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}
/**
* @param {number} number
* @param {boolean | undefined} lowercase
* @returns {string}
*/
module.exports.toRomanNumerals = function(number, lowercase) {
    const retptr = globalArgumentPtr();
    wasm.toRomanNumerals(retptr, number, isLikeNone(lowercase) ? 0xFFFFFF : lowercase ? 1 : 0);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getStringFromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;

};

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
* @param {Uint8Array} bytes
* @returns {number}
*/
module.exports.computeAdler32 = function(bytes) {
    const ptr0 = passArray8ToWasm(bytes);
    const len0 = WASM_VECTOR_LEN;
    try {
        return wasm.computeAdler32(ptr0, len0) >>> 0;

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

};

wasm = require('./pdfjs_bg');

