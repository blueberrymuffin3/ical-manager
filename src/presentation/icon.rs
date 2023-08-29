use icondata::IconData;
use maud::{html, Markup, PreEscaped};

pub fn icon(data: impl Into<IconData>) -> Markup {
    icon_base(data, None)
}

pub fn icon_alt(data: impl Into<IconData>, alt: &str) -> Markup {
    icon_base(data, Some(alt))
}

fn icon_base(data: impl Into<IconData>, alt: Option<&str>) -> Markup {
    let data = data.into();

    html!(
        svg
            class="icon"
            style=[data.style]
            x=[data.x]
            y=[data.y]
            viewBox=[data.view_box]
            stroke-linecap=[data.stroke_linecap]
            stroke-linejoin=[data.stroke_linejoin]
            stroke-width=[data.stroke_width]
            stroke=(data.stroke.unwrap_or("currentColor"))
            fill=(data.fill.unwrap_or("currentColor"))
            role="graphics-symbol"
        {
            @if let Some(alt) = alt {
                title { (alt) }
            }
            (PreEscaped(data.data))
        }
    )
}
