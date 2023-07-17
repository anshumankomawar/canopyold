<script>
  import { RailCard } from '$lib';
  let railCards = [
    { text: 'RailCard 1' },
    { text: 'RailCard 2' },
    { text: 'RailCard 3' }
  ];
  export let selectedItem = '';

  function handleClick(item) {
    console.log("handling click")
    selectedItem = item;
    // Emit a custom event to notify the parent component
    dispatch('itemSelected', item);
  }
</script>

<ul>
  {#each railCards as railCard}
    <li>
      <RailCard text={railCard.text} isSelected={railCard.text === selectedItem} on:click={() => handleClick(railCard.text)} />
    </li>
  {/each}
</ul>
  
<style>
  nav {
    width: 200px;
    background-color: #f1f1f1;
  }
</style>
