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

  let lists = [
    {
      "id":1,
      "name":"shopping",
      "tasks":[
        {
          "id":1,
          "task":"eggs",
          "details":null,
          "completed":false
        }
      ]
    }
  ];
  const td_utf8 = new TextDecoder("utf-8");
  fetch("/list").then((res) => (res.body.getReader().read()))
  .then((res) => {lists = JSON.parse(td_utf8.decode(res.value));});
</script>
<p>{string}</p>

{#each lists as list}
    <List list={list}/>
{/each}
<NewList/>