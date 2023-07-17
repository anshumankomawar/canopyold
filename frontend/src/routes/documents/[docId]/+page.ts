import Document from "$lib/models/Document";

export async function load({ params }) {
    const url = `http://localhost:3000/bdoc/get/${params.docId}`;

    const response = await fetch(url, {
        method: "GET",
        mode: 'cors',
        headers: {
        "Content-Type": "application/json"
        }
    });

  
    if (!response.ok) {
      throw new Error("Failed to load data from the API");
    }
  
    const responseData = await response.json();
  
    return {
        document: {
            id: params.docId,
            title: `Title for ${params.docId} goes here`,
            content: responseData.result.content
        }
    };
  }
  