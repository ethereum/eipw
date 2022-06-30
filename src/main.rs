/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::Snippet;

use clap::{Parser, ValueEnum};

use eipw_lint::reporters::{Json, Reporter, Text};
use eipw_lint::{default_lints, Linter};

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;

use tokio::fs;

#[derive(Debug, Parser)]
struct Opts {
    /// List all available lints.
    #[clap(exclusive(true), long)]
    list_lints: bool,

    /// Files and/or directories to check.
    #[clap(required_unless_present("list-lints"))]
    sources: Vec<PathBuf>,

    /// Output format.
    #[clap(long, value_enum, default_value_t)]
    format: Format,

    /// Do not enable the default lints.
    #[clap(long, requires_all(&["lints"]))]
    no_default_lints: bool,

    /// Additional lints to enable.
    #[clap(long, value_delimiter(','))]
    lints: Vec<String>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Text,
    Json,
}

impl Default for Format {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Debug)]
enum EitherReporter {
    Json(Json),
    Text(Text<String>),
}

impl Reporter for EitherReporter {
    fn report(&self, snippet: Snippet<'_>) -> Result<(), eipw_lint::reporters::Error> {
        match self {
            Self::Json(j) => j.report(snippet),
            Self::Text(s) => s.report(snippet),
        }
    }
}

fn list_lints() {
    println!("Available lints:");

    for (slug, _) in default_lints() {
        println!("\t{}", slug);
    }

    println!();
}

async fn collect_sources(sources: Vec<PathBuf>) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut output = Vec::with_capacity(sources.len());

    for source in sources.into_iter() {
        let metadata = fs::metadata(&source).await?;
        if metadata.is_file() {
            output.push(source.clone());
        }

        if !metadata.is_dir() {
            continue;
        }

        let mut entries = fs::read_dir(&source).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = fs::metadata(&path).await?;
            if !metadata.is_file() {
                continue;
            }

            if path.extension() != Some(OsStr::new("md")) {
                continue;
            }

            output.push(path);
        }
    }

    Ok(output)
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    if opts.list_lints {
        list_lints();
        return;
    }

    let stdout = std::io::stdout();

    let sources = collect_sources(opts.sources).await.unwrap();

    for source in sources {
        let reporter = match opts.format {
            Format::Json => EitherReporter::Json(Json::default()),
            Format::Text => EitherReporter::Text(Text::default()),
        };

        let origin = source.to_string_lossy();
        let mut linter = Linter::new(reporter).origin(&origin);

        if opts.no_default_lints {
            linter = linter.clear_lints();
        }

        if !opts.lints.is_empty() {
            let lints: HashMap<_, _> = default_lints().collect();
            for (slug, lint) in lints {
                linter = linter.add_lint(slug, lint);
            }
        }

        let text = fs::read_to_string(&source).await.unwrap();

        let reporter = linter.check(&text).await.unwrap();

        // TODO: The json output isn't valid when parsing multiple files.

        match reporter {
            EitherReporter::Json(j) => serde_json::to_writer_pretty(&stdout, &j).unwrap(),
            EitherReporter::Text(t) => println!("{}", t.into_inner()),
        }
    }
}
