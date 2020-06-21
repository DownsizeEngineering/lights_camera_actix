<script>
let newListName;
const handleSubmit = function() {
  alert(`sending a new list named ${newListName}!`);
}
</script> 
<h1>New List:</h1>
<form on:submit|preventDefault={handleSubmit}>
  <label>Name</label>
  <input type="text" bind:value={newListName}/>
</form>