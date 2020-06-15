<script>
    let string = "Hello World"
    
import("../wasm/hello_wasm.js")
    .then(module => {
        module.greet();
        string = "What a nice day!"
    }
);
</script>
<p>{string}</p>