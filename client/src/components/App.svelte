<script>
  import TodoLists from './TodoLists.svelte';

  let string = "Hello World"
  import("../wasm/hello_wasm.js")
    .then(module => {
      module.greet();
      string = "What a nice day!"
    }
  );

  const fetchLists = new Promise((resolve, reject) => {
    const td_utf8 = new TextDecoder("utf-8");
    fetch("/list").then((res) => (res.body.getReader().read()))
    .catch((error) => reject(error))
    .then((res) => resolve(JSON.parse(td_utf8.decode(res.value))))
    .catch((error) => reject(error));
  });

</script>

<p>{string}</p>

{#await fetchLists}
  <p>loading lists...</p>
{:then lists}
  <TodoLists lists={lists}/>
{:catch error}
<p>{error.message}</p>
{/await}