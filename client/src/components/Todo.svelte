<script>
  import sanitizeHTML from '../Sanitize';
  export let todo;

  let update = function(status) {
    let uri = `./todo/${todo.id}?status=${status}`;
    fetch(uri, {method: 'PATCH'});
  };

  todo.task = sanitizeHTML(todo.task);
  todo.details = sanitizeHTML(todo.details);

  let checkboxBind = function() {
    todo.completed = !todo.completed;
    update(todo.completed);
  }
  checkboxBind = checkboxBind.bind(this);
</script>

  {todo.task}
  <input type=checkbox bind:checked={todo.completed}
   on:click={checkboxBind}/>
