<script>
  import Todo from './Todo.svelte';
  import NewTodo from './NewTodo.svelte';
  import sanitizeHTML from '../Sanitize';
  export let list;

  list.name = sanitizeHTML(list.name);
</script>

<ol>
<lh>{list.name}</lh>
{#each list.tasks as todo}
<Todo todo={todo}/>
{/each}
</ol>
<NewTodo bind:list={list}/>