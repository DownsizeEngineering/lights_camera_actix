<script>
  import List from './List.svelte';
  import NewList from './NewList.svelte';
  let string = "Hello World"
  import("../wasm/hello_wasm.js")
    .then(module => {
      module.greet();
      string = "What a nice day!"
    }
  );

  const td_utf8 = new TextDecoder("utf-8");

  const newList = function(name) {
    if (name == "") return;
    let list = {
      name,
      tasks: [],
    }
    lists = [...lists, list]

    name = name.replace(/'/g, "\\'");
    let uri = `/list?name=${encodeURIComponent(name)}`;

    fetch(uri, {method: 'POST'}).then((res) => (res.body.getReader().read()))
    .then((res) => {list.id = parseInt(td_utf8.decode(res.value));});
  }

  const fetchLists = new Promise((resolve, reject) => {
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
  {#each lists as list}
      <List list={list}/>
  {/each}
  <NewList post={newList.bind(this)}/>
{:catch error}
<p>{error.message}</p>
{/await}