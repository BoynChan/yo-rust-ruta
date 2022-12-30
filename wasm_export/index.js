import init from "./pkg/wasm_export.js"

const runWasm = async () => {
    // Instantiate our wasm module
    const rustWasm = await init("./pkg/wasm_export_bg.wasm");

    // Call the Add function export from wasm, save the result
    const result = rustWasm.call_me_from_javascript(24, 24);

    console.log(result); // Should output '72'
    console.log(rustWasm.ADD_CONSTANT); // Should output 'undefined'
    console.log(rustWasm.add_integer_with_constant); // Should output 'undefined'
};

export function f() {
    console.log("call f")
}
runWasm();