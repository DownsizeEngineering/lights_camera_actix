<script>
  export let list;
  let task = "";
  let details = "";
 
  let newTodo = function(task, details) {
    if (task === "") return;
    let todo = {
      task,
      details: details ? details : "",
    }
    list.tasks = [...list.tasks, todo];

    task = task.replace(/'/g, "\\'");
    if (details) details = details.replace(/'/g, "\\'");

    let uri = `/list/${list.id}?id=0&task=${encodeURIComponent(task)}`;
    if (details) uri += `&details=${encodeURIComponent(details)}`;
    uri += `&completed=false`;
    
    let td_utf8 = new TextDecoder("utf-8"); 
    fetch(uri, {method: 'POST'}).then((res) => (res.body.getReader().read()))
    .then((res) => {todo.id = parseInt(td_utf8.decode(res.value));});
  }

   const handleSubmit = function() {
    newTodo(task, details);
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