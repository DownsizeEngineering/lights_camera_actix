<script>
  import Todo from './Todo.svelte';
  import NewTodo from './NewTodo.svelte';
  export let list;

  let newTodo = function(task, details) {
    if (task === "") return;
    let todo = {
      task,
      details: details ? details : null,
    }
    list.tasks = [...list.tasks, todo];
    let uri = `/list/${list.id}?id=0&task=${encodeURIComponent(todo.task)}`;
    if (details) uri = uri + `&details=${encodeURIComponent(todo.details)}`;
    let td_utf8 = new TextDecoder("utf-8"); 
    fetch(uri, {method: 'POST'}).then((res) => (res.body.getReader().read()))
    .then((res) => {todo.id = parseInt(td_utf8.decode(res.value))});
  }
</script>

<ol>
<lh>{list.name}</lh>
{#each list.tasks as todo}
<Todo todo={todo}/>
{/each}
</ol>
<NewTodo post={newTodo.bind(this)}/>