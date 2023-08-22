/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct NoDuplicates;

impl Lint for NoDuplicates {
    fn lint<'a>(&self, slug: &'a str, ctx: &Context<'a, '_>) -> Result<(), Error> {
        let mut defined = HashMap::new();

        for field in ctx.preamble().fields() {
            match defined.entry(field.name()) {
                Entry::Vacant(v) => {
                    v.insert(field);
                }
                Entry::Occupied(o) => {
                    let original = o.get();
                    let original_count = original.source().chars().count();
                    let field_count = field.source().chars().count();
                    let label = format!(
                        "preamble header `{}` defined multiple times",
                        original.name()
                    );
                    ctx.report(Snippet {
                        title: Some(Annotation {
                            id: Some(slug),
                            annotation_type: ctx.annotation_type(),
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
                                    range: (0, original_count),
                                }],
                            },
                            Slice {
                                line_start: field.line_start(),
                                fold: false,
                                origin: ctx.origin(),
                                source: field.source(),
                                annotations: vec![SourceAnnotation {
                                    annotation_type: ctx.annotation_type(),
                                    label: "redefined here",
                                    range: (0, field_count),
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
