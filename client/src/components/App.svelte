<script>
    import List from './List.svelte'
    let string = "Hello World"
    import("../wasm/hello_wasm.js")
        .then(module => {
            module.greet();
            string = "What a nice day!"
        }
    );

    let lists = [{"id":1,"name":"shopping","tasks":[{"id":1,"task":"eggs","details":null,"completed":false}]},{"id":2,"name":"banana","tasks":[]},{"id":3,"name":"banana","tasks":[]}];
</script>
<p>{string}</p>

{#each lists as list}
    <List list={list}/>
{/each}