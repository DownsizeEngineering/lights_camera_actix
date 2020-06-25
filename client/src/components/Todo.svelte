<script>
  import sanitizeHTML from '../Sanitize';
  export let todo;
  let complete = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAABUUlEQVRIS72VYVEDMRCFvyoABxQFBQVQB6CgdQBVQFEAKAAUQBUUCeAAHIACOm8m6ezt7fWSMkdm8uuy79u87O6NGHiNBtanBDAHzoCTtJXTe9qvwGpXkrsAF8AdMO655SewAARrrS7APXBVaZ9iBGqsCLCPeBZ9AK4twQNky0tl5v74pbXLA+Tn0R8B0jjOGhaganmsEP8BZOdNELO9hQU8AbNCgMTPU6kuA8gzoIQbfaDanhQArLh6Yw0cujhpnXrAbyB+m6riIH0rEW/Yby3yAInr+sryLUVlW7oyb1WoBXiLvoFp8lmCWjpTIv6Rx0rfI1uIACXiOhc+cleTZYgCoweN6iIsUx3sajRBtHy1ROJfdkD++6hQRoMOu3zlfSCtSeobzfupRxeob/jJc43oqh+OhQmkrRLNo0R1rp6QaCgcTdOCMVR/pOSnX69qIjbmMUYZlWQeyQAAAABJRU5ErkJggg==";
  let incomplete = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAABY0lEQVRIS72V4TFFMRCFv1cBHaACVIAOqAAVoBJUgApQAR2gAzqgAuaYrNm7d5P78mauzORPbnK+ze7Z3AUzj8XM+iwDOAH2gJ0yFdNrmQ/AYyvIFuAQuAQ2J275DlwAgo1GDXAFnHWmT2cEGowMsIq4iV4D554QAUrLfWfkcfuRT1cEKJ8bCeALWAvr2Zq2SGPL9nqA3HKTiJ8Wxzw7iMT3i6uyM3+38IBb4DgAPoGDApBNBdGQuKyqtSdgPZy7AxTwoA90YDu5QYRYH9TE7ftuBHw3iush2tYSH6Tfp6gFsJzrlgbwNcli+9X2gFqKvLgi9ymqQd7sWZkqssSUS19QrfnCvyThp0WuNZncpQ71brGaqGuj88RLbWpNkjVao/6jTx/+gfz3p0LhzPrY2X1XgYxe0mjTmEwVXaCpmijnKnbXD8fDBNJUD9hTIp/LuhJNhQft3GOR3r3L/PR7NQf7fwBDLk0ZyC9d1QAAAABJRU5ErkJggg==";
  let clickable = true;

  let update = function(status) {
    if (clickable === false) {return}
    clickable = false;
    todo.completed = status;

    let uri = `./todo/${todo.id}?status=${status}`;
    fetch(uri, {method: 'PATCH'}).then((res) => {
      if (res.status === 200) clickable=true;
      });
  };

  let markComplete = update.bind(this, true);
  let markIncomplete = update.bind(this, false);

  todo.task = sanitizeHTML(todo.task);
  todo.details = sanitizeHTML(todo.details);
</script>

<li>
  {todo.task}
  {#if todo.completed}
    <img alt="complete" src={complete} on:click={markIncomplete}/> 
  {:else}
    <img alt="incomplete" src={incomplete} on:click={markComplete}/>
  {/if}
</li>
