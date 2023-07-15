<script>
    import { navigate } from 'svelte-routing';

    export let cardData = {};
    let title = cardData.title;
    let description = cardData.description;
    let link = cardData.link;
  

    function handleClick() {
        navigate(`/docs/${encodeURIComponent(title)}`, { state: { cardData } });
    }

    function handleKeyDown(event) {
        if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            navigate(`/docs/${encodeURIComponent(title)}`, { state: { cardData } });
        }
    }
  </script>
  
  <button class="card" on:click={handleClick} on:keydown={handleKeyDown}>
    <h3>{title}</h3>
    <p>{description}</p>
    <a href={link} class="link">Click here for more information</a>
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
  
    .link {
      display: none; /* Hide the link since the entire card is clickable */
    }
  </style>
  