/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug)]
pub struct NoDuplicates;

impl Lint for NoDuplicates {
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        let mut defined = HashMap::new();

        for field in ctx.preamble().fields() {
            match defined.entry(field.name()) {
                Entry::Vacant(v) => {
                    v.insert(field);
                }
                Entry::Occupied(o) => {
                    let original = o.get();
                    let label = format!(
                        "preamble header `{}` defined multiple times",
                        original.name()
                    );
                    ctx.report(Snippet {
                        title: Some(Annotation {
                            id: Some(slug),
                            annotation_type: AnnotationType::Error,
                            label: Some(&label),
                        }),
                        footer: vec![],
                        slices: vec![
                            Slice {
                                line_start: original.line_start(),
                                fold: false,
                                origin: ctx.origin(),
                                source: original.source(),
                                annotations: vec![SourceAnnotation {
                                    annotation_type: AnnotationType::Info,
                                    label: "first defined here",
                                    range: (0, original.source().len()),
                                }],
                            },
                            Slice {
                                line_start: field.line_start(),
                                fold: false,
                                origin: ctx.origin(),
                                source: field.source(),
                                annotations: vec![SourceAnnotation {
                                    annotation_type: AnnotationType::Error,
                                    label: "redefined here",
                                    range: (0, field.source().len()),
                                }],
                            },
                        ],
                        opt: Default::default(),
                    })?;
                }
            }
        }

        Ok(())
    }
}
