<script>
  let task = "";
  let details = "";
</script> 
<h1>New Todo:</h1>
<form>
  <label>Name</label>
  <input type="text" bind:value={task}/>
  <br>
  <label>Description</label>
  <input type="text" bind:value={details}/>
</form>