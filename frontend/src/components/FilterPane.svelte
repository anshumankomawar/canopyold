<script>
    export let filters = {};
  
    function toggleFilter(filter) {
      filters[filter] = !filters[filter];
      dispatch('filter', filters);
    }
  </script>
  
  <div id="filter-pane">
    {#each Object.keys(filters) as filter (filter)}
      <label>
        <input type="checkbox" bind:checked={filters[filter]} on:change={() => toggleFilter(filter)} />
        {filter}
      </label>
    {/each}
  </div>