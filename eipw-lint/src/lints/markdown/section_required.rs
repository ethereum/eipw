/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

use comrak::nodes::{Ast, NodeHeading, NodeValue};

use crate::lints::{Context, Error, Lint};

use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SectionRequired<S>(pub Vec<S>);

impl<S> Lint for SectionRequired<S>
where
    S: Debug + Display + AsRef<str> + Clone + PartialEq<String>,
{
    fn lint<'a, 'b>(&self, slug: &'a str, ctx: &Context<'a, 'b>) -> Result<(), Error> {
        // Collect the headings.
        let headings: Vec<_> = ctx
            .body()
            .descendants()
            // Find all headings of level 2.
            .filter(|start| {
                matches!(
                    &*start.data.borrow(),
                    Ast {
                        value: NodeValue::Heading(NodeHeading { level: 2, .. }),
                        ..
                    }
                )
            })
            // Descend into their children.
            .map(|heading| {
                let collected = heading
                    .descendants()
                    .skip(1)
                    // Filter for text nodes.
                    .filter_map(|child| match &*child.data.borrow() {
                        Ast {
                            value: NodeValue::Text(v),
                            ..
                        } => Some(v.to_owned()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                collected
            })
            .collect();

        // Use a `Vec` here to preserve the order of sections.
        let mut missing = self.0.to_vec();

        // TODO: I'm sure this is horribly inefficient!
        missing.retain(|i| {
            for text in &headings {
                if i == text {
                    return false;
                }
            }
            true
        });

        if missing.is_empty() {
            return Ok(());
        }

        let missing_txt = missing
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>()
            .join("`, `");
        let label = format!("body is missing section(s): `{}`", missing_txt);
        ctx.report(Snippet {
            title: Some(Annotation {
                annotation_type: ctx.annotation_type(),
                id: Some(slug),
                label: Some(&label),
            }),
            slices: vec![Slice {
                fold: true,
                annotations: vec![],
                origin: ctx.origin(),
                source: ctx.body_source(),
                line_start: ctx.body().data.borrow().sourcepos.start.line,
            }],
            footer: vec![Annotation {
                id: None,
                label: Some("must be at the second level (`## Heading`)"),
                annotation_type: AnnotationType::Help,
            }],
            opt: Default::default(),
        })?;

        Ok(())
    }
}
