<script>
  import List from './List.svelte';
  import NewList from './NewList.svelte';
  export let lists;
</script>

{#each lists as list}
      <List list={list}/>
{/each}
  <NewList bind:lists={lists}/>