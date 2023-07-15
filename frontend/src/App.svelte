<!-- App.svelte -->
  <script>
	import SearchBar from './components/SearchBar.svelte';
	import ResultList from './components/ResultList.svelte';
	import FilterPane from './components/FilterPane.svelte';
  
	let results = [];
	let filters = {};
  
	function handleSearch(event) {
	  results = event.detail; // Update with your own API call
	}
  
	function handleFilter(event) {
	  filters = event.detail; // Update with your own filtering logic
	}
  </script>
  
  <div id="App">
	<header>
	  <SearchBar on:search={handleSearch} />
	</header>
  
	<main>
	  <ResultList {results} />
	  <FilterPane on:filter={handleFilter} {filters} /> 
	</main>
  </div>