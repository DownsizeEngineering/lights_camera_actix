<script>
  export let post;
  let task = "";
  let details = "";
  const handleSubmit = function() {
    post(task, details);
    task = "";
    details = "";
  }

</script> 
<h1>New Todo:</h1>
<form on:submit|preventDefault={handleSubmit.bind(this)}>
  <label>Name</label>
  <input type="text" bind:value={task}/>
  <br>
  <label>Description</label>
  <input type="text" bind:value={details}/>
  <button>Submit</button>
</form>