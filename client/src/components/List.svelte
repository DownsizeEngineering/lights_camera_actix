<script>
  import Todo from './Todo.svelte';
  import NewTodo from './NewTodo.svelte';
  import sanitizeHTML from '../Sanitize';
  export let list;

  list.name = sanitizeHTML(list.name);

  const deleteTodo = function (todo) {
    let {id} = todo;
    fetch(`/todo/${id}`, {method:'DELETE'});
    list.tasks.splice(list.tasks.indexOf(todo), 1);
    list = list;
  }
</script>

<style>
  li:nth-child(even) {
    background-color: grey;
  }
  li:nth-child(odd){
    background-color: lightgrey;
  }
  button {
    background-color: red;
  }
</style>

<ol>
<lh>{list.name}</lh>
{#each list.tasks as todo}
<li>
  <button on:click={deleteTodo.bind(this, todo)}>x</button>
  <Todo todo={todo}/>
</li>
{/each}
</ol>
<NewTodo bind:list={list}/>