<script>
    import data from './scraped_data.json';
</script>

<h2>{data.header}</h2>

{#each data.paragraphs as paragraph}
  <p>{paragraph}</p>
{/each}