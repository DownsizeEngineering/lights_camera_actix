import App from "./App.svelte";
import("./pkg/hello_wasm.js").then(module => module.greet());

const app = new App( {target: document.body} );
