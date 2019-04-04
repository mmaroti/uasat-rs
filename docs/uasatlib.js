import * as wasm from './uasatlib_bg.wasm';

/**
* @param {number} a
* @param {number} b
* @returns {number}
*/
export function uasat_test(a, b) {
    return wasm.uasat_test(a, b);
}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

export function __wbindgen_object_drop_ref(i) { dropObject(i); }

