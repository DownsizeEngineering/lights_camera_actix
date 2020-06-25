<script>
  import List from './List.svelte';
  import NewList from './NewList.svelte';
  export let lists;

  const deleteList = function(list) {
    let {id} = list;
    fetch(`/list/${id}`, {method:'DELETE'});
    lists.splice(lists.indexOf(list), 1);
    lists = lists;
  }
</script>

  {#each lists as list}
    <button on:click={deleteList.bind(this, list)}>x</button>
    <List list={list}/>
  {/each}
  <NewList bind:lists={lists}/>