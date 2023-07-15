<script>

  import { goto } from '$app/navigation';

  export let cardData = {};
  let title = cardData.title;
  let description = cardData.description;
  let link = cardData.link;

  function handleClick() {
        goto(`${encodeURIComponent(title)}`);
    }

    function handleKeyDown(event) {
        if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            goto(`${encodeURIComponent(title)}`);
        }
  }
</script>

<button class="card" on:click={handleClick} on:keydown={handleKeyDown}>
  <h3>{title}</h3>
  <p>{description}</p>
</button>

<style>
  .card {
    border: 1px solid #ccc;
    padding: 10px;
    margin-bottom: 10px;
    cursor: pointer;
    display: block;
    background: none;
    border: none;
    text-align: left;
  }

  h3 {
    font-size: 18px;
    margin-top: 0;
  }

  p {
    margin-bottom: 10px;
  }

</style>
