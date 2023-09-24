use icondata::LuIcon;
use maud::{html, Markup, DOCTYPE};

use crate::{data::user::User, presentation::icon::icon_alt};

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

                script src="/js/main.js" {}
                script src="/js/htmx.min.js" {}
                script src="/js/idiomorph-ext.min.js" {}
            }
        }
    }
}

pub fn layout_no_user(content: Markup) -> Markup {
    base_layout(html!(
        div."medium-container" {
            header {
                h1 {
                    a href="/" {
                        "iCal Manager"
                    }
                }
            }
            (content)
        }
    ))
}

pub fn layout_user(user: &User, content: Markup) -> Markup {
    let img_url = user
        .data
        .icon
        .as_deref()
        .unwrap_or("/img/default_profile.svg");

    let name = user.data.name.as_deref().unwrap_or("Unknown");

    base_layout(html!(
        div."medium-container" {
            header {
                h1 {
                    a href="/" {
                        "iCal Manager"
                    }
                }

                ."user-info" {
                    span { (name) }
                    img src=(img_url) {}
                    button.small-button.danger-button hx-post="/login/logout" {
                        (icon_alt(LuIcon::LuLogOut, "Logout"))
                    }
                }
            }
            (content)
        }
    ))
}
