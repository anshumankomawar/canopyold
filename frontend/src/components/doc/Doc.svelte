<script>
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
  
    let cardData = {};
  
    onMount(() => {
      const unsubscribe = page.subscribe($page => {
        cardData = $page?.state?.cardData || {};
      });
  
      return unsubscribe;
    });
  </script>
  
  <div>
    <h1>{cardData.title}</h1>
    <p>{cardData.description}</p>
  </div>