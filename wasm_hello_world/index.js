import init from "./pkg/wasm_hello_world.js"

const helloWorld = await init("./pkg/wasm_hello_world_bg.wasm")
console.log("hello world loaded")
function addTwo() {
    const [i1, i2] = [document.getElementById("i1").textContent, document.getElementById("i2").textContent]
    const addResult = helloWorld.add(parseInt(i1), parseInt(i2))
    document.body.textContent = `Hello World! AddResult:${addResult}`
}
//
// const runWasm = async () => {
//     const helloWorld = await init("./pkg/wasm_hello_world_bg.wasm")
//     const addResult = helloWorld.add(24,24)
//     document.body.textContent = `Hello World! AddResult:${addResult}`
// };
// runWasm();