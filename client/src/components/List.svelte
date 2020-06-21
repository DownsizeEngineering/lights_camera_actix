<script>
  import Todo from './Todo.svelte';
  import NewTodo from './NewTodo.svelte';
  export let list;
</script>

<ol>
<lh>{list.name}</lh>
{#each list.tasks as todo}
<Todo todo={todo}/>
{/each}
</ol>
<li><NewTodo/></li>