use maud::{html, Markup, DOCTYPE};

pub fn base_layout(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta http-equiv="x-ua-compatible" content="ie=edge";
                meta name="viewport" content="width=device-width, initial-scale=1";

                title { "iCal Manager" }

                link rel="stylesheet" href="/css/primitive.css";
                link rel="stylesheet" href="/css/main.css";
                link rel="icon" href="/images/favicon.png";
            }
            body {
                (content)

                script src="/js/htmx.min.js" {}
                script src="/js/main.js" {}
            }
        }
    }
}
pub fn layout(content: Markup) -> Markup {
    base_layout(html!(
        div."medium-container" {
            h1 {
                a href="/" {
                    "iCal Manager"
                }
            }
            (content)
        }
    ))
}
