<script>
  export let text;
  export let isSelected = false;
  import { mdiStar } from '@mdi/js';
</script>

<div class="rail-card" class:selected={isSelected}>
  <div class="icon">
    <svg class="icon-svg" viewBox="0 0 24 24">
        <path d={mdiStar} />
    </svg>
  </div>
  <p>{text}</p>
</div>

<style>
  .rail-card {
    display: flex;
    align-items: center;
    background-color: #f1f1f1;
    padding: 8px;
    margin-bottom: 8px;
    border-radius: 4px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    transition: background-color 0.3s;
    cursor: pointer;
  }

  .rail-card[selected] {
    background-color: #e0e0e0;
  }

  .rail-card:hover {
    background-color: #e0e0e0;
  }

  .icon {
    display: flex;
    align-items: center;
    margin-right: 8px;
  }

  .icon-svg {
    width: 1em;
    height: 1em;
  }

  p {
    margin: 0;
  }
</style>
