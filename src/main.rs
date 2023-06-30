/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::Snippet;

use clap::{Parser, ValueEnum};

use eipw_lint::lints::{DefaultLint, Lint};
use eipw_lint::reporters::count::Count;
use eipw_lint::reporters::{AdditionalHelp, Json, Reporter, Text};
use eipw_lint::{default_lints, Linter};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
struct Opts {
    /// List all available lints.
    #[clap(exclusive(true), long)]
    list_lints: bool,

    /// Files and/or directories to check.
    #[clap(required_unless_present("list_lints"))]
    sources: Vec<PathBuf>,

    /// Output format.
    #[clap(long, value_enum, default_value_t)]
    format: Format,

    /// Do not enable the default lints.
    #[clap(long)]
    no_default_lints: bool,

    /// Lints to enable as errors.
    #[clap(long, short('D'))]
    deny: Vec<String>,

    /// Lints to enable as warnings.
    #[clap(long, short('W'))]
    warn: Vec<String>,

    /// Lints to disable.
    #[clap(long, short('A'))]
    allow: Vec<String>,

    /// Path to file defining alternate default lints.
    #[clap(long, short('c'))]
    config: Option<PathBuf>,
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

#[cfg(target_arch = "wasm32")]
async fn read_config(_path: &Path) -> Lints {
    todo!()
}

#[cfg(not(target_arch = "wasm32"))]
async fn read_config(path: &Path) -> Lints {
    let contents = tokio::fs::read_to_string(path)
        .await
        .expect("couldn't read config file");

    toml::from_str(&contents).expect("couldn't parse config file")
}

#[cfg(target_arch = "wasm32")]
async fn collect_sources(_sources: Vec<PathBuf>) -> Result<Vec<PathBuf>, std::io::Error> {
    todo!()
}

#[cfg(not(target_arch = "wasm32"))]
async fn collect_sources(sources: Vec<PathBuf>) -> Result<Vec<PathBuf>, std::io::Error> {
    use std::ffi::OsStr;
    use tokio::fs;

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

#[derive(Debug, Serialize, Deserialize)]
struct Lints {
    lints: HashMap<String, DefaultLint<String>>,
}

#[cfg_attr(target_arch = "wasm32", tokio::main(flavor = "current_thread"))]
#[cfg_attr(not(target_arch = "wasm32"), tokio::main)]
async fn run(opts: Opts) -> Result<(), usize> {
    if opts.list_lints {
        list_lints();
        return Ok(());
    }

    let stdout = std::io::stdout();

    let sources = collect_sources(opts.sources).await.unwrap();

    let reporter = match opts.format {
        Format::Json => EitherReporter::Json(Json::default()),
        Format::Text => EitherReporter::Text(Text::default()),
    };

    let reporter = AdditionalHelp::new(reporter, |t: &str| {
        Ok(format!("see https://ethereum.github.io/eipw/{}/", t))
    });
    let reporter = Count::new(reporter);

    let lints: Lints;
    let mut linter;
    if let Some(ref path) = opts.config {
        lints = read_config(path).await;
        linter = Linter::with_lints(
            reporter,
            lints
                .lints
                .iter()
                .map(|(k, v)| (k.as_str(), Box::new(v.clone()) as Box<dyn Lint>)),
        );
    } else {
        linter = Linter::new(reporter);
    }

    if opts.no_default_lints {
        linter = linter.clear_lints();
    }

    for allow in opts.allow {
        linter = linter.allow(&allow);
    }

    if !opts.warn.is_empty() {
        let mut lints: HashMap<_, _> = default_lints().collect();
        for warn in opts.warn {
            let (k, v) = lints.remove_entry(warn.as_str()).unwrap();
            linter = linter.warn(k, v);
        }
    }

    if !opts.deny.is_empty() {
        let mut lints: HashMap<_, _> = default_lints().collect();
        for deny in opts.deny {
            let (k, v) = lints.remove_entry(deny.as_str()).unwrap();
            linter = linter.deny(k, v);
        }
    }

    for source in &sources {
        linter = linter.check_file(source);
    }

    let reporter = linter.run().await.unwrap();

    let n_errors = reporter.counts().error;

    match reporter.into_inner().into_inner() {
        EitherReporter::Json(j) => serde_json::to_writer_pretty(&stdout, &j).unwrap(),
        EitherReporter::Text(t) => print!("{}", t.into_inner()),
    }

    if n_errors > 0 {
        Err(n_errors)
    } else {
        Ok(())
    }
}

fn main() {
    let opts = Opts::parse();

    if let Err(n_errors) = run(opts) {
        eprintln!("validation failed with {} errors :(", n_errors);
        std::process::exit(1);
    }
}
