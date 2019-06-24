let wasm;

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

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
