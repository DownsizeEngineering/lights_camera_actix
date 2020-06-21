<script>
  export let listId;
  let task = "";
  let details = "";
  const handleSubmit = function() {
    alert(`sending a new todo named ${task} for list ${listId}!`);
  }
</script> 
<h1>New Todo:</h1>
<form on:submit|preventDefault={handleSubmit}>
  <label>Name</label>
  <input type="text" bind:value={task}/>
  <br>
  <label>Description</label>
  <input type="text" bind:value={details}/>
  <button>Submit</button>
</form>