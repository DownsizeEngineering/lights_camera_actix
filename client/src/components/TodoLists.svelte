<script>
  import { onMount } from 'svelte';
  import List from './List.svelte';
  import NewList from './NewList.svelte';

  let lists;

  onMount(async () => {
    const res = await fetch('/list');
    lists = await res.json();
  });


  const deleteList = function(list) {
    let {id} = list;
    fetch(`/list/${id}`, {method:'DELETE'});
    lists.splice(lists.indexOf(list), 1);
    lists = lists;
  }
</script>

<style>
  button {
    background-color: red;
    border: none;
  }

  .list {
    width: 300px;
    border: 1px solid black;
  }

  .todoLists {
    display: flex;
    flex-wrap: wrap;
  }

  .todoLists * {
    display: block;
  }
</style>

  {#if lists === undefined}
  <p>loading lists...</p>
  {:else}
    <div class="todoLists">
    {#each lists as list}
    <div class="list">
      <button on:click={deleteList.bind(this, list)}>x</button>
      <List list={list}/>
    </div>
    {/each}
    <NewList bind:lists={lists}/>
    </div>
{/if}

