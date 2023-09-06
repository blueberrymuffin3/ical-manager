use axum_typed_multipart::{TryFromField, TryFromMultipart};
use icondata::LuIcon;
use maud::{html, Markup, Render};
use strum::IntoStaticStr;

use crate::{
    data::{
        feed::FeedData,
        filters::Filters,
        source::{Source, UrlSource},
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
    fn add(&mut self, field: &'static str, error: String) {
        self.0.push(ValidationError { field, error });
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
                        (error.error)
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
    pub source_ttl: String,
}

impl From<FeedData> for FeedFormValues {
    fn from(value: FeedData) -> Self {
        let (source_type, source_link, source_ttl) = match value.source {
            Source::UrlSource(UrlSource { source_link, ttl }) => {
                (FormFeedSourceType::UrlSource, source_link, ttl.to_string())
            }
            Source::FileSource => (FormFeedSourceType::FileSource, String::new(), String::new()),
        };

        Self {
            name: value.name,
            source_type,
            source_link,
            source_ttl,
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
                let source = UrlSource {
                    source_link: self.source_link.to_string(),
                    ttl: errors.unwrap_or_default("source-ttl", self.source_ttl.parse()),
                };
                if source.source_link.is_empty() {
                    errors.add("source-link", "Link must not be empty".to_owned());
                }

                Source::UrlSource(source)
            }
            FormFeedSourceType::FileSource => Source::FileSource,
        };

        if errors.is_empty() {
            Ok(FeedData {
                name,
                source,
                filters: Filters::default(),
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

pub fn feed_form(values: Option<&FeedFormValues>, errors: ValidationErrors) -> Markup {
    let (name, feed_type, link, ttl) = match values {
        Some(FeedFormValues {
            name,
            source_type,
            source_link,
            source_ttl,
        }) => (
            Some(name.as_str()),
            Some(*source_type),
            Some(source_link.as_str()),
            source_ttl.as_str(),
        ),
        None => (None, None, None, DURATION_6_HOURS),
    };

    html!(
        form #"feed-form" name="feed-form" hx-post="" hx-ext="morph" hx-swap="morph:outerHTML" enctype="multipart/form-data" {
            h3 {"Source"}

            label for="name" {"Name"}
            input #"name" name="name" type="text" value=[name];
            (errors.render("name"))

            label for="source-type" {"Type"}
            select #"source-type" name="source-type" onchange=r#"updateSelectGroups("feed-form", "source-type");"# value=[feed_type.map(IntoStrumStr::into_strum_str)] {
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

                label for="source-ttl" {"Minimum Update Period"}
                input #"source-ttl" name="source-ttl" type="text" value=(ttl) ;
                (errors.render("source-ttl"))
                p {"Enter a value like 1 day, 15min, 3h, etc..."}
            }

            .hide data-show-for-id="source-type" data-show-for-value=(FormFeedSourceType::FileSource.into_strum_str()) {
                label for="source-upload" {"Upload iCal"}
                input #"source-upload" name="source-upload" type="file";
                (errors.render("source-upload"))
            }

            h3 {"Filters"}
            label for="filter-cr" {
                input #"filter-cr" name="filter-cr" type="checkbox";
                " Remove Carriage Returns (" code {"\\r"} ")"
                (errors.render("filter-cr"))
            }

            (submit_button("Submit"))
        }

    )
}
