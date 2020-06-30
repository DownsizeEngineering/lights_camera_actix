<script>
  export let lists;
  let newListName = "";

  const handleSubmit = function() {
    newList(newListName);
    newListName = "";
  }

  const newList = function(name) {
    if (name == "") return;
    const td_utf8 = new TextDecoder("utf-8");
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
</script> 

<style>
div {
  display: block;
}
</style>
<div>
  <h1>New List:</h1>
  <form on:submit|preventDefault={handleSubmit}>
    <label>Name</label>
    <input type="text" bind:value={newListName}/>
</form>
</div>