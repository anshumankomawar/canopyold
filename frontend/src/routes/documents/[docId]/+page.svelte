<script>
    import { page } from '$app/stores'
    import EditorComponent from "../../../lib/business_doc/MainPage.svelte";

    const docId = $page.params.docId
    export let data;
</script>

<EditorComponent content={data.document.content} id={docId}/>