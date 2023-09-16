use anyhow::anyhow;
use async_trait::async_trait;
use axum_typed_multipart::{TryFromField, TryFromMultipart, TypedMultipartError};
use bytes::Bytes;
use icondata::LuIcon;
use itertools::intersperse;
use maud::{html, Markup, Render};
use strum::IntoStaticStr;

#[derive(Debug)]
pub struct MultipartCheckbox;
#[async_trait]
impl TryFromField for MultipartCheckbox {
    async fn try_from_field(
        _field: axum::extract::multipart::Field<'_>,
        _limit_bytes: Option<usize>,
    ) -> Result<Self, TypedMultipartError> {
        return Ok(Self);
    }
}

trait BoolEquiv {
    fn as_bool(&self) -> bool;
    fn from_bool(b: bool) -> Self;
}

impl BoolEquiv for Option<MultipartCheckbox> {
    fn as_bool(&self) -> bool {
        self.is_some()
    }

    fn from_bool(b: bool) -> Self {
        match b {
            true => Some(MultipartCheckbox),
            false => None,
        }
    }
}

use crate::{
    data::{
        feed::{FeedData, FeedUpdateError},
        filters::{Filter, Filters},
        source::{Source, SourceFile, SourceHTTP},
    },
    presentation::icon::icon,
    strum_util::IntoStrumStr,
};

#[derive(Debug)]
pub struct ValidationError {
    field: &'static str,
    error: String,
}

#[derive(Debug, Default)]
pub struct ValidationErrors(Vec<ValidationError>);
impl ValidationErrors {
    pub fn add(&mut self, field: &'static str, error: String) {
        self.0.push(ValidationError { field, error });
    }

    fn add_general(&mut self, error: String) {
        self.add("general", error)
    }

    pub fn from_feed_update_error(error: FeedUpdateError) -> ValidationErrors {
        let mut this = Self::default();

        let s = |e: FeedUpdateError| format!("{:?}", anyhow!(e));

        match error {
            FeedUpdateError::FileSourceMissingFileError => this.add("source-upload", s(error)),
            other => this.add_general(s(other)),
        }

        this
    }

    fn unwrap_or_default<T: Default>(
        &mut self,
        field: &'static str,
        result: Result<T, impl ToString>,
    ) -> T {
        match result {
            Ok(t) => t,
            Err(error) => {
                self.add(field, error.to_string());
                T::default()
            }
        }
    }

    fn render<'a>(&'a self, field: &'static str) -> impl Render + 'a {
        ValidationErrorsRenderer {
            errors: self,
            field,
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

struct ValidationErrorsRenderer<'a> {
    errors: &'a ValidationErrors,
    field: &'static str,
}

impl<'a> maud::Render for ValidationErrorsRenderer<'a> {
    fn render_to(&self, buffer: &mut String) {
        for error in &self.errors.0 {
            if error.field == self.field {
                html!(
                    p."validation-error" {
                        @for line in intersperse(
                            error.error
                                .lines()
                                .map(|line| {
                                    html!((line))
                                }),
                            html!(br;)
                        ) {
                            (line)
                        }
                    }
                )
                .render_to(buffer)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoStaticStr, TryFromField)]
#[strum(serialize_all = "kebab-case")]
#[try_from_field(rename_all = "kebab-case")]
pub enum FormFeedSourceType {
    UrlSource,
    FileSource,
}

#[derive(Debug, TryFromMultipart)]
#[try_from_multipart(strict = true, rename_all = "kebab-case")]
pub struct FeedFormValues {
    pub name: String,
    pub source_type: FormFeedSourceType,
    pub source_link: String,
    pub source_upload: Option<Bytes>,
    // pub source_ttl: String,
    // pub filter_cr: Option<MultipartCheckbox>,
}

impl From<FeedData> for FeedFormValues {
    fn from(value: FeedData) -> Self {
        let (source_type, source_link, source_upload) = match value.source {
            Source::HTTP(SourceHTTP { link }) => (FormFeedSourceType::UrlSource, link, None),
            Source::File(SourceFile { contents }) => {
                (FormFeedSourceType::FileSource, String::new(), contents)
            }
        };

        // let filter_cr = value
        //     .filters
        //     .iter()
        //     .any(|filter| matches!(filter, Filter::RemoveCarriageReturn));

        Self {
            name: value.name,
            source_type,
            source_link,
            // source_ttl,
            source_upload,
            // filter_cr: BoolEquiv::from_bool(filter_cr),
        }
    }
}

impl FeedFormValues {
    pub fn try_into_feed_data(&self) -> Result<FeedData, ValidationErrors> {
        let mut errors = ValidationErrors::default();

        let name = self.name.trim().to_string();
        if name.is_empty() {
            errors.add("name", "Name must not be empty".to_owned());
        }

        let source = match self.source_type {
            FormFeedSourceType::UrlSource => {
                let source = SourceHTTP {
                    link: self.source_link.to_string(),
                    // ttl: errors.unwrap_or_default("source-ttl", self.source_ttl.parse()),
                };
                if source.link.is_empty() {
                    errors.add("source-link", "Link must not be empty".to_owned());
                }

                Source::HTTP(source)
            }
            FormFeedSourceType::FileSource => Source::File(SourceFile {
                contents: self.source_upload.clone(),
            }),
        };

        let mut filters = Filters::default();
        // if self.filter_cr.as_bool() {
        //     filters.0.push(Filter::RemoveCarriageReturn);
        // }

        if errors.is_empty() {
            Ok(FeedData {
                name,
                source,
                filters,
            })
        } else {
            Err(errors)
        }
    }
}

fn select_options<'a, T: Eq + 'a>(selected_value: Option<&T>, options: &'a [(T, &str)]) -> Markup
where
    &'static str: From<&'a T>,
{
    html!(
        @for (value, label) in options {
            option value=(value.into_strum_str()) selected[selected_value == Some(value)] {(label)}
        }
    )
}

fn submit_button(content: impl Render) -> Markup {
    html!(
        button."spinner-button" {
            (content)
            (icon(LuIcon::LuRefreshCw))
        }
    )
}

const DURATION_6_HOURS: &str = "6h";

pub fn feed_form(
    values: Option<&FeedFormValues>,
    is_new: bool,
    errors: ValidationErrors,
) -> Markup {
    let (name, feed_type, link) = match values {
        Some(FeedFormValues {
            name,
            source_type,
            source_link,
            // source_ttl,
            source_upload: _,
            // filter_cr,
        }) => (
            Some(name.as_str()),
            Some(*source_type),
            Some(source_link.as_str()),
            // source_ttl.as_str(),
            // filter_cr.as_bool(),
        ),
        None => (None, None, None),
    };

    let submit_text = match is_new {
        true => "Create",
        false => "Save",
    };

    html!(
        form #"feed-form" name="feed-form" hx-post="" hx-ext="morph" hx-swap="morph:outerHTML" enctype="multipart/form-data" {
            h3 {"Source"}

            label for="name" {"Name"}
            input #"name" name="name" type="text" value=[name];
            (errors.render("name"))

            label for="source-type" {"Type"}
            select
                #"source-type"
                name="source-type"
                value=[feed_type.map(IntoStrumStr::into_strum_str)]
                data-trigger-show-hide-form="feed-form"
            {
                (select_options(
                    feed_type.as_ref(),
                &[
                    (FormFeedSourceType::UrlSource, "Fetch URL"),
                    (FormFeedSourceType::FileSource, "File Upload"),
                ]))
            }
            (errors.render("source-type"))

            .hide data-show-for-id="source-type" data-show-for-value=(FormFeedSourceType::UrlSource.into_strum_str()) {
                label for="source-link" {"Link"}
                input #"source-link" name="source-link" type="text" placeholder="https://example.com/feed.ical" value=[link];
                (errors.render("source-link"))

                // label for="source-ttl" {"Minimum Update Period"}
                // input #"source-ttl" name="source-ttl" type="text" value=(ttl) ;
                // (errors.render("source-ttl"))
                // p {"Enter a value like 1 day, 15min, 3h, etc..."}
            }

            .hide data-show-for-id="source-type" data-show-for-value=(FormFeedSourceType::FileSource.into_strum_str()) {
                label for="source-upload" {"Upload iCal"}
                input #"source-upload" name="source-upload" type="file";
                (errors.render("source-upload"))
            }

            h3 {"Filters"}
            // label for="filter-cr" {
            //     input #"filter-cr" name="filter-cr" type="checkbox" checked[filter_cr];
            //     " Remove Carriage Returns (" code {"\\r"} ")"
            //     (errors.render("filter-cr"))
            // }

            (submit_button(submit_text))
            (errors.render("general"))
        }
    )
}
