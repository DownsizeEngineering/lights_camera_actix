<script>
  import Todo from './Todo.svelte';
  export let list;
</script>

<ol>
<lh>{list.name}</lh>
{#each list.tasks as todo}
<Todo todo={todo}/>
{/each}
</ol>