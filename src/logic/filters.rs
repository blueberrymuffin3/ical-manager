use anyhow::anyhow;
use bytes::Bytes;
use icalendar::parser::{unfold, Calendar};

use crate::data::filters::{Filter, Filters};

#[derive(Debug)]
pub struct FilterStats {
    pub event_count: usize,
    pub size: usize,
}

pub fn apply_filters(data: Bytes, filters: &Filters) -> anyhow::Result<(Bytes, FilterStats)> {
    let data_string = std::str::from_utf8(data.as_ref())?;
    let data_string = unfold(data_string);
    let data_string = filters.apply_pre_parse(data_string)?;

    let mut calendar = parse(&data_string)?;

    filters.apply_post_parse(&mut calendar)?;

    let event_count = calendar.components.len();
    let bytes: Bytes = calendar.to_string().into_bytes().into();
    let stats = FilterStats {
        event_count,
        size: bytes.len(),
    };

    Ok((bytes, stats))
}

fn parse(data_string: &str) -> Result<icalendar::parser::Calendar<'_>, anyhow::Error> {
    icalendar::parser::read_calendar(data_string).map_err(|x| anyhow!(x))
}

trait FilterTrait {
    fn apply_pre_parse(&self, text: String) -> anyhow::Result<String> {
        Ok(text)
    }

    #[allow(unused_variables)]
    fn apply_post_parse(&self, calendar: &mut Calendar<'_>) -> anyhow::Result<()> {
        Ok(())
    }
}

impl FilterTrait for Filters {
    fn apply_pre_parse(&self, mut text: String) -> anyhow::Result<String> {
        for filter in self.iter() {
            text = filter.apply_pre_parse(text)?;
        }
        Ok(text)
    }

    fn apply_post_parse(&self, calendar: &mut Calendar<'_>) -> anyhow::Result<()> {
        for filter in self.iter() {
            filter.apply_post_parse(calendar)?;
        }
        Ok(())
    }
}

impl FilterTrait for Filter {
    fn apply_pre_parse(&self, text: String) -> anyhow::Result<String> {
        match self {
            _ => Ok(text),
        }
    }
}
