//! provides functionality to work with content stored on disk
#![allow(unused_imports)]

use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;
use yaml_front_matter::Document;
use chrono::{Local, DateTime, Utc};

fn list_md_files(dir: &str) -> Result<Vec<fs::DirEntry>, io::Error> {
    let entries = fs::read_dir(dir)?;

    let mut md_entries = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("md")) {
            println!("{}", path.display());
            md_entries.push(entry);
        }
    }

    Ok(md_entries)
}

fn get_md_file_metadata(path: &str) -> Result<Metadata, io::Error> {
    let file_content = fs::read_to_string(path)?;
    let document: Document<Metadata> = YamlFrontMatter::parse::<Metadata>(&file_content).unwrap();
    Ok(document.metadata)
}

fn list_md_files_metadata(dir: &str) -> Result<Vec<DocEntry>, io::Error> {
    let md_entries = list_md_files(dir)?;

    let mut doc_entries: Vec<DocEntry> = Vec::new();

    for entry in md_entries {
        let path = entry.path();
        let metadata = get_md_file_metadata(path.to_str().unwrap())?;
        let doc_entry = DocEntry {
            path,
            metadata,
        };
        doc_entries.push(doc_entry);
    }

    doc_entries.sort_by(|a, b| a.metadata.date_created.cmp(&b.metadata.date_created));

    Ok(doc_entries)
}

#[derive(Deserialize, PartialEq, Debug)]
enum DocType {
    Text,
    Video,
    Audio,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    title: String,
    doc_type: DocType,
    tags: Vec<String>,
    date_created: DateTime::<Utc>,
    date_modified: DateTime::<Utc>,
}

#[derive(Debug)]
struct DocEntry {
    path: PathBuf,
    metadata: Metadata,
}

const SAMPLE_MD_DOC: &str = r#"
---
title: 'Livre premier--Un juste, Chapitre I'
doc_type: 'Text'
tags: ['tag1', 'tag2']
date_created: '2014-11-28T12:45:59.324310806Z'
date_modified: '2015-11-28T12:45:59.324310806Z'
---

Livre premier--Un juste

Chapitre I

Monsieur Myriel

En 1815, M. Charles-François-Bienvenu Myriel était évêque de Digne.
C'était un vieillard d'environ soixante-quinze ans; il occupait le siège
de Digne depuis 1806.

"#;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_md_files() {
        let result = list_md_files("/Users/chaosarium/Desktop/influx_content/fr");
        assert!(result.is_ok());
    }

    #[test]
    fn test_frontmatter_extract() {
        
        let document: Document<Metadata> = YamlFrontMatter::parse::<Metadata>(&SAMPLE_MD_DOC).unwrap();

        assert_eq!(document.metadata.title, "Livre premier--Un juste, Chapitre I");
        assert_eq!(document.metadata.doc_type, DocType::Text);
        assert_eq!(document.metadata.tags, vec!["tag1", "tag2"]);
        assert_eq!(document.metadata.date_created, "2014-11-28T12:45:59.324310806Z".parse::<DateTime<Utc>>().unwrap());
        assert_eq!(document.metadata.date_modified, "2015-11-28T12:45:59.324310806Z".parse::<DateTime<Utc>>().unwrap());

        println!("{}", document.content)
    }
        
    #[test]
    fn test_get_md_file_metadata() {
        let path = "/Users/chaosarium/Desktop/influx_content/fr/Les misérables 1.md";
        let result = get_md_file_metadata(path);
        
        assert!(result.is_ok());
        let metadata = result.unwrap();
        
        // Add assertions for the expected metadata values
        assert_eq!(metadata.title, "Livre premier--Un juste, Chapitre I");
        assert_eq!(metadata.doc_type, DocType::Text);
        assert_eq!(metadata.tags, vec!["tag1", "tag2"]);
        assert_eq!(metadata.date_created, "2014-11-28T12:45:59.324310806Z".parse::<DateTime<Utc>>().unwrap());
        assert_eq!(metadata.date_modified, "2015-11-28T12:45:59.324310806Z".parse::<DateTime<Utc>>().unwrap());
    }

        
    #[test]
    fn test_list_md_files_metadata() {
        let directory = "/Users/chaosarium/Desktop/influx_content/fr";
        let result = list_md_files_metadata(directory);

        assert!(result.is_ok());
        let metadata_list = result.unwrap();

        assert_eq!(metadata_list.len(), 3);
        println!("{:#?}", metadata_list);

        let expected_titles = vec![
            "Livre premier--Un juste, Chapitre I",
            "Livre premier--Un juste, Chapitre II",
            "Livre premier--Un juste, Chapitre III",
        ];
        let expected_doc_types = vec![
            DocType::Text,
            DocType::Text,
            DocType::Text,
        ];
        let expected_tags = vec![
            vec!["tag1", "tag2"],
            vec!["tag2", "tag3"],
            vec!["tag3"],
        ];

        for (i, DocEntry {path, metadata}) in metadata_list.iter().enumerate() {
            assert_eq!(metadata.title, expected_titles[i]);
            assert_eq!(metadata.doc_type, expected_doc_types[i]);
            assert_eq!(metadata.tags, expected_tags[i]);
        }

    }


}