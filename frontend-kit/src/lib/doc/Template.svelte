<script>
    export let cardData = {};
    import data from './components/scraped_data.json';
    import Block from './components/Block.svelte';
</script>
  
<div>
    <h1>{cardData.title}</h1>
    <p>{cardData.description}</p>
</div>

<Block />