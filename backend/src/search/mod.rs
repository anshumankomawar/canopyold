use std::path::Path;
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

#[derive(Clone)]
pub struct SearchEngine {
    pub index: Index,
    pub reader: IndexReader,
    pub schema: Schema,
    pub fields: Vec<Field>,
}

impl SearchEngine {
    pub fn search(&self, query: &str) -> Result<Vec<(Score, DocAddress)>> {
        let query_parser = QueryParser::for_index(&self.index, self.fields.clone());
        let query = match query_parser.parse_query(&format!("{}*", query).to_string()) {
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

    pub fn add_document(&self, topic: &str) -> Result<()> {
        let mut index_writer = self.index.writer(50_000_000).unwrap();
        let doc = doc!(
            self.fields[0] => topic,
            self.fields[1] => "",
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
    let topics = schema_builder.add_text_field("topics", TEXT | STORED);
    let resources = schema_builder.add_text_field("resources", TEXT);
    let schema = schema_builder.build();

    if !Path::new("index").exists() {
        std::fs::create_dir("index").unwrap();
    }
    let dir = MmapDirectory::open("index").unwrap();
    let index = Index::open_or_create(dir, schema.clone()).unwrap();

    let reader = index
        .reader_builder()
        .reload_policy(tantivy::ReloadPolicy::OnCommit)
        .try_into()
        .unwrap();

    SearchEngine {
        index,
        reader,
        schema,
        fields: vec![topics, resources],
    }
}
