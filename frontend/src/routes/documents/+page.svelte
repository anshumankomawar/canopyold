<!-- DocumentList.svelte -->
<script>
    import { goto } from '$app/navigation';
    // Sample list of documents
    let documents = [
      { id: 1, title: 'Document 1' },
      { id: 2, title: 'Document 2' },
      { id: 3, title: 'Document 3' }
    ];

    function navigateToDocument(documentID) {
    goto(`/documents/${documentID}`);
  }
  </script>
  
  {#each documents as document}
    <div class="document-card" on:click={() => navigateToDocument(document.id)}>
      <h2 class="document-title">{document.title}</h2>
      <!-- Add additional content for the document card -->
    </div>
  {/each}
  
  <style>
    .document-card {
      display: flex;
      flex-direction: column;
      align-items: center;
      width: 200px;
      padding: 16px;
      border: 1px solid #ccc;
      border-radius: 4px;
      cursor: pointer;
      margin-bottom: 16px;
    }
  
    .document-title {
      font-weight: bold;
      margin-bottom: 8px;
    }
  </style>