// This file is derived from `rust-lang/annotate-snippets-rs` at revision
// `542e41e9a767c1c294564d549c46b2c3974b6481`, and is available under the
// Apache-2.0 and MIT licenses.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use annotate_snippets::{
    display_list::{FormatOptions, Margin},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};

mod annotation_opt {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Annotation<'de>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper<'a>(
            #[serde(with = "AnnotationDef")]
            #[serde(borrow)]
            Annotation<'a>,
        );

        Option::<Wrapper>::deserialize(deserializer)
            .map(|opt_wrapped: Option<Wrapper>| opt_wrapped.map(|wrapped: Wrapper| wrapped.0))
    }

    pub fn serialize<S>(v: &Option<Annotation<'_>>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Wrapper<'a>(
            #[serde(with = "AnnotationDef")]
            #[serde(borrow)]
            &'a Annotation<'a>,
        );

        v.as_ref().map(Wrapper).serialize(ser)
    }
}

mod annotation_vec {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Annotation<'de>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper<'a>(
            #[serde(with = "AnnotationDef")]
            #[serde(borrow)]
            Annotation<'a>,
        );

        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|Wrapper(a)| a).collect())
    }

    pub fn serialize<S>(v: &[Annotation<'_>], ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Wrapper<'a>(
            #[serde(with = "AnnotationDef")]
            #[serde(borrow)]
            &'a Annotation<'a>,
        );

        v.iter().map(Wrapper).collect::<Vec<_>>().serialize(ser)
    }
}

mod options {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<FormatOptions, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper(#[serde(with = "FormatOptionsDef")] FormatOptions);

        Wrapper::deserialize(deserializer).map(|w| w.0)
    }

    pub fn serialize<S>(v: &FormatOptions, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Wrapper<'a>(#[serde(with = "FormatOptionsDef")] &'a FormatOptions);

        Wrapper(v).serialize(ser)
    }
}

mod slice_vec {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Slice<'de>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper<'a>(
            #[serde(with = "SliceDef")]
            #[serde(borrow)]
            Slice<'a>,
        );

        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|Wrapper(a)| a).collect())
    }

    pub fn serialize<S>(v: &[Slice<'_>], ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Wrapper<'a>(
            #[serde(with = "SliceDef")]
            #[serde(borrow)]
            &'a Slice<'a>,
        );

        v.iter().map(Wrapper).collect::<Vec<_>>().serialize(ser)
    }
}

mod source_annotation_vec {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<SourceAnnotation<'de>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper<'a>(
            #[serde(with = "SourceAnnotationDef")]
            #[serde(borrow)]
            SourceAnnotation<'a>,
        );

        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|Wrapper(a)| a).collect())
    }

    pub fn serialize<S>(v: &[SourceAnnotation<'_>], ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Wrapper<'a>(
            #[serde(with = "SourceAnnotationDef")]
            #[serde(borrow)]
            &'a SourceAnnotation<'a>,
        );

        v.iter().map(Wrapper).collect::<Vec<_>>().serialize(ser)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SnippetDef<'a> {
    #[serde(with = "annotation_opt")]
    #[serde(default)]
    #[serde(borrow)]
    pub title: Option<Annotation<'a>>,
    #[serde(with = "annotation_vec")]
    #[serde(default)]
    #[serde(borrow)]
    pub footer: Vec<Annotation<'a>>,
    #[serde(with = "options")]
    #[serde(default)]
    pub opt: FormatOptions,
    #[serde(with = "slice_vec")]
    #[serde(borrow)]
    pub slices: Vec<Slice<'a>>,
}

impl<'a> From<SnippetDef<'a>> for Snippet<'a> {
    fn from(def: SnippetDef<'a>) -> Self {
        let SnippetDef {
            title,
            footer,
            opt,
            slices,
        } = def;
        Snippet {
            title,
            footer,
            slices,
            opt,
        }
    }
}

impl<'a> From<Snippet<'a>> for SnippetDef<'a> {
    fn from(s: Snippet<'a>) -> Self {
        Self {
            title: s.title,
            footer: s.footer,
            opt: s.opt,
            slices: s.slices,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "FormatOptions")]
pub struct FormatOptionsDef {
    #[serde(default)]
    pub color: bool,
    #[serde(default)]
    pub anonymized_line_numbers: bool,
    #[serde(skip, default)]
    pub margin: Option<Margin>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Slice")]
pub struct SliceDef<'a> {
    #[serde(borrow)]
    pub source: &'a str,
    pub line_start: usize,
    #[serde(borrow)]
    pub origin: Option<&'a str>,
    #[serde(with = "source_annotation_vec")]
    #[serde(borrow)]
    pub annotations: Vec<SourceAnnotation<'a>>,
    #[serde(default)]
    pub fold: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "SourceAnnotation")]
pub struct SourceAnnotationDef<'a> {
    pub range: (usize, usize),
    #[serde(borrow)]
    pub label: &'a str,
    #[serde(with = "AnnotationTypeDef")]
    pub annotation_type: AnnotationType,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Annotation")]
pub struct AnnotationDef<'a> {
    #[serde(borrow)]
    pub id: Option<&'a str>,
    #[serde(borrow)]
    pub label: Option<&'a str>,
    #[serde(with = "AnnotationTypeDef")]
    pub annotation_type: AnnotationType,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "AnnotationType")]
enum AnnotationTypeDef {
    Error,
    Warning,
    Info,
    Note,
    Help,
}
