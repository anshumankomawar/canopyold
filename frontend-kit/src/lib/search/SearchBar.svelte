<script>
    import SearchCard from './SearchCard.svelte';

    let searchData = [];
    let searchText = '';
  
    // Dummy test data
    let dummyData = [
    {
        "id":1,
        "title": "Explore the Historical Sites of Rome",
        "description": "Immerse yourself in the rich history and culture of Rome. Visit iconic landmarks such as the Colosseum, Vatican City, and the Roman Forum. Discover the wonders of ancient architecture and experience the charm of the Eternal City.",
        "link": "https://www.example.com/rome-historical-sites"
    },
    {
        "id":2,
        "title": "Indulge in a Culinary Journey in Paris",
        "description": "Savor the flavors of French cuisine in the culinary capital, Paris. Delight in mouthwatering pastries, indulge in gourmet meals, and explore local food markets. Experience the art of gastronomy in the city of love and lights.",
        "link": "https://www.example.com/paris-culinary-journey"
    },
    {
        "id":3,
        "title": "Escape to the Tranquil Beaches of Bali",
        "description": "Relax and unwind on the pristine beaches of Bali. Soak up the sun, take a dip in crystal-clear waters, and experience breathtaking sunsets. Immerse yourself in the tranquility of this tropical paradise and rejuvenate your senses.",
        "link": "https://www.example.com/bali-beaches"
    }];
  
    function search() {
      searchData = dummyData.filter(item =>
        item.title.toLowerCase().includes(searchText.toLowerCase())
      );
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
    placeholder="Search..."
    bind:value={searchText}
    on:input={search}   
  />

  <button class="search-button" on:click={search}>Search</button>
</div>

<div>
  <h2>Search Results:</h2>
  {#each searchData as result (result.title)}
    <SearchCard 
      cardData={result}
    />
  {/each}
</div>
