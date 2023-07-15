<script>
    export let results = [];
  </script>
  
  <div id="result-list">
    {#each results as result (result.id)}
      <div class="card">
        <h2>{result.title}</h2>
        <p>{result.description}</p>
      </div>
    {/each}
  </div>