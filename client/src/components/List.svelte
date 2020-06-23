<script>
  import Todo from './Todo.svelte';
  import NewTodo from './NewTodo.svelte';
  export let list;

  let newTodo = function(task, details) {
    if (task === "") return;
    let todo = {
      task,
      details: details ? details : "",
    }
    list.tasks = [...list.tasks, todo];

    let uri = `/list/${list.id}?id=0&task=${encodeURIComponent(task)}`;
    if (details) uri += `&details=${encodeURIComponent(details)}`;
    uri += `&completed=false`;
    
    let td_utf8 = new TextDecoder("utf-8"); 
    fetch(uri, {method: 'POST'}).then((res) => (res.body.getReader().read()))
    .then((res) => {todo.id = parseInt(td_utf8.decode(res.value));});
  }
</script>

<ol>
<lh>{list.name}</lh>
{#each list.tasks as todo}
<Todo todo={todo}/>
{/each}
</ol>
<NewTodo post={newTodo.bind(this)}/>