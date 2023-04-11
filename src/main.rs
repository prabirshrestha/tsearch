use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;
use ignore::Walk;
use rayon::prelude::*;
use stopwatch::Stopwatch;
use tree_sitter::{Language, Query, QueryCapture, QueryCursor};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short = 'p', long = "path")]
    path: PathBuf,

    #[arg(short = 'q', long = "quiet", default_value = "false")]
    quiet: bool,

    #[arg(last = true, required = true)]
    pattern: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let sw = Stopwatch::start_new();

    let cli = Cli::parse();

    let full_pattern = format!("{} @full_pattern_cli_capture", cli.pattern.join(" ")); // Add an extra root pattern to force capturing the root pattern for display.

    let mut paths = Vec::new();

    if !cli.quiet {
        println!("Getting file list");
    }

    for entry in Walk::new(&cli.path) {
        let entry = entry?;

        let path = entry.into_path();
        if path.is_file() {
            let ext = path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();

            if matches!(ext, "ts" | "tsx" | "rs") {
                paths.push(path.clone());
            }
        }
    }

    if !cli.quiet {
        println!("Getting file list complete");

        println!("Searching {} file(s).\n", paths.len());
    }

    paths.par_iter().for_each(|path| {
        let language = get_langauge(path);
        if language.is_none() {
            return;
        }

        let query = Query::new(language.unwrap(), &full_pattern)
            .expect("Error building query from given string.");

        let mut parser = tree_sitter::Parser::new();
        parser.set_language(language.unwrap()).unwrap();

        let source_code = fs::read_to_string(path).unwrap();
        let tree = parser.parse(&source_code, None).unwrap();

        let mut cursor = QueryCursor::new();
        let root_node = tree.root_node();

        let source_bytes = source_code.as_bytes();

        let mut seen_nodes: HashSet<usize> = HashSet::new();

        for m in cursor.matches(&query, root_node, source_bytes) {
            let captures: HashMap<_, _> = m
                .captures
                .iter()
                .map(|c: &QueryCapture| (query.capture_names()[c.index as usize].clone(), c))
                .collect();

            // This is the capture we added above so we :can access the root node of the query match.
            let full_capture = captures["full_pattern_cli_capture"];

            if seen_nodes.contains(&full_capture.node.id()) {
                continue; // Don't consider at the same node twice. Sometimes the same node can match multiple times.
            }

            seen_nodes.insert(full_capture.node.id());

            for capture in captures {
                let result = format!(
                    "{}:{}:{}",
                    path.display(),
                    capture.1.node.start_position().row + 1,
                    capture.1.node.start_position().column,
                );
                println!("{result}");
            }
        }
    });

    if !cli.quiet {
        println!("Total time: {}ms", sw.elapsed_ms());
    }

    Ok(())
}

fn get_langauge(path: &Path) -> Option<Language> {
    match path.extension().unwrap_or_default().to_str() {
        Some("tsx") => Some(tree_sitter_typescript::language_tsx()),
        Some("ts") => Some(tree_sitter_typescript::language_typescript()),
        Some("rs") => Some(tree_sitter_rust::language()),
        _ => None,
    }
}
