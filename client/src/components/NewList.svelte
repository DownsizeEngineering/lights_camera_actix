<script>
let newListName;
export let post;
const handleSubmit = function() {
  post(newListName);
}
</script> 
<h1>New List:</h1>
<form on:submit|preventDefault={handleSubmit}>
  <label>Name</label>
  <input type="text" bind:value={newListName}/>
</form>