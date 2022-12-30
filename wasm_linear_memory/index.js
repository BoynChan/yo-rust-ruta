import init from "./pkg/wasm_linear_memory.js"

const runWasm = async () => {
    // Instantiate our wasm module
    const rustWasm = await init("./pkg/wasm_linear_memory_bg.wasm");

    rustWasm.store_value_in_wasm_memory_buffer_index_zero(24)

    const pointer = rustWasm.get_wasm_memory_buffer_pointer()

    const memory = new Uint8Array(rustWasm.memory.buffer)

    console.log("pointer",pointer)
    // should be 24
    console.log("m[0]",memory[pointer])
    // should be 0
    console.log("m[1]",memory[pointer+1])

    memory[pointer+1] = 15

    // should be 15
    console.log("m[1] after change",memory[pointer+1])
    console.log("get index one",rustWasm.read_wasm_memory_buffer_and_return_index_one())
};
runWasm();