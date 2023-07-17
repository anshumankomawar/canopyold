use std::path::Path;
use tantivy::schema::{IndexRecordOption, TextFieldIndexing, TextOptions};
use tantivy::DocAddress;
use tantivy::{
    collector::TopDocs,
    doc,
    query::QueryParser,
    schema::{Schema, SchemaBuilder, STORED, TEXT},
    Index, IndexReader, Score,
};
use tantivy::{directory::MmapDirectory, schema::Field};

use crate::error::{Error, Result};
use crate::models::topic::TopicTable;

#[derive(Clone)]
pub struct SearchEngine {
    pub index: Index,
    pub reader: IndexReader,
    pub schema: Schema,
    pub fields: Vec<Field>,
    pub query_fields: Vec<Field>,
}

impl SearchEngine {
    fn prefix_field(field: String, query: &str) -> String {
        format!("{}:{}", field, query)
    }

    pub fn search(&self, query: &str) -> Result<Vec<(Score, DocAddress)>> {
        let query_parser = QueryParser::for_index(&self.index, self.query_fields.clone());
        let query = match query_parser
            .parse_query(&Self::prefix_field("Paragraph.content".to_string(), query))
        {
            Ok(query) => query,
            Err(e) => return Err(Error::QueryParseError { error: e }),
        };
        let searcher = &self.reader.searcher();
        match searcher.search(&query, &TopDocs::with_limit(10)) {
            Ok(docs) => Ok(docs),
            Err(e) => Err(Error::SearchError { error: e }),
        }
    }

    pub fn convert_to_json(&self, docs: Vec<(Score, DocAddress)>) -> Vec<String> {
        let mut results = Vec::new();
        for (_score, doc_address) in docs {
            let searcher = &self.reader.searcher();
            let retrieved_doc = searcher.doc(doc_address).unwrap();
            results.push(self.schema.to_json(&retrieved_doc));
        }
        results
    }

    pub fn add_document(&self, topic: TopicTable) -> Result<()> {
        let mut index_writer = self.index.writer(50_000_000).unwrap();
        let doc = doc!(
            self.fields[0] => topic.id,
            self.fields[1] => topic.name.to_string(),
            self.fields[2] => topic.description.to_string(),
            self.fields[3] => topic.get_components_json(),
        );
        match index_writer.add_document(doc) {
            Ok(_) => match index_writer.commit() {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::IndexError { error: e }),
            },
            Err(e) => return Err(Error::IndexError { error: e }),
        }
    }
}

pub fn get_search_engine() -> SearchEngine {
    let mut schema_builder = SchemaBuilder::new();
    let topic_id = schema_builder.add_u64_field("topic_id", STORED);
    let topic_name = schema_builder.add_text_field("topic_name", TEXT | STORED);
    let topic_description = schema_builder.add_text_field("topic_description", TEXT);

    let index_options = TextFieldIndexing::default().set_index_option(IndexRecordOption::WithFreqs);
    let text_options = TextOptions::default().set_indexing_options(index_options);
    let topic_body = schema_builder.add_json_field("topic_body", text_options);

    let schema = schema_builder.build();

    if !Path::new("index").exists() {
        std::fs::create_dir("index").unwrap();
    }
    let dir = MmapDirectory::open("index").unwrap();
    println!("Opening index...");
    let index = Index::open_or_create(dir, schema.clone()).unwrap();
    println!("Index: {:?}", index);

    let reader = index
        .reader_builder()
        .reload_policy(tantivy::ReloadPolicy::OnCommit)
        .try_into()
        .unwrap();

    SearchEngine {
        index,
        reader,
        schema,
        fields: vec![topic_id, topic_name, topic_description, topic_body],
        query_fields: vec![topic_name, topic_description, topic_body],
    }
}
