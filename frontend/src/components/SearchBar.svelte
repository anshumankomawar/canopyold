  <script>
    // SearchBar.svelte
    let searchQuery = '';
  
    let dummyData = [
      { id: 1, name: 'John Doe' },
      { id: 2, name: 'Jane Smith' },
      { id: 3, name: 'David Johnson' },
      { id: 4, name: 'Emily Davis' },
      { id: 5, name: 'Michael Wilson' }
    ];
  
    let searchResults = [];
  
    function handleSearch() {
      searchResults = dummyData.filter(item => item.name.toLowerCase().includes(searchQuery.toLowerCase()));
    }
  </script>
  
  <style>
    .search-container {
      display: flex;
      justify-content: center;
      align-items: center;
    }
  
    .search-input {
      padding: 8px;
      font-size: 16px;
      border: 1px solid #ccc;
      border-radius: 4px;
      margin-right: 8px;
    }
  
    .search-button {
      padding: 8px 16px;
      background-color: #007bff;
      color: #fff;
      border: none;
      border-radius: 4px;
      cursor: pointer;
    }
  </style>
  
  <div class="search-container">
    <input
      class="search-input"
      type="text"
      placeholder="Enter your search query"
      bind:value={searchQuery}
    />
  
    <button class="search-button" on:click={handleSearch}>Search</button>
  </div>
  
  <div>
    <h2>Search Results:</h2>
    {#each searchResults as result (result.id)}
      <p>{result.name}</p>
    {/each}
  </div>
  