#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod logic {
    use axum::{
        extract::{Path, State},
        http::StatusCode, response::{IntoResponse, Response},
    };
    use maud::html;
    use sqlx::SqlitePool;
    use crate::presentation::error::{make_error_page, ServerResult};
    pub async fn export(
        State(pool): State<SqlitePool>,
        Path(mut code): Path<String>,
    ) -> ServerResult<Response> {
        if let Some(dot) = code.find('.') {
            code.truncate(dot);
        }
        code.make_ascii_lowercase();
        let Some(feed) = {
            {
                #[allow(clippy::all)]
                {
                    use ::sqlx::Arguments as _;
                    let arg0 = &(code);
                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::HasArguments>::Arguments::default();
                    query_args
                        .reserve(
                            1usize,
                            0
                                + ::sqlx::encode::Encode::<
                                    sqlx::sqlite::Sqlite,
                                >::size_hint(arg0),
                        );
                    query_args.add(arg0);
                    struct Record {
                        id: i64,
                        name: String,
                        link_code: String,
                        filters: Vec<u8>,
                        source_link: ::std::option::Option<String>,
                        ttl_seconds: ::std::option::Option<i64>,
                        last_update: ::std::option::Option<i64>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for Record {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let names: &'static _ = &[
                                "id",
                                "name",
                                "link_code",
                                "filters",
                                "source_link",
                                "ttl_seconds",
                                "last_update",
                            ];
                            let values: &[&dyn ::core::fmt::Debug] = &[
                                &self.id,
                                &self.name,
                                &self.link_code,
                                &self.filters,
                                &self.source_link,
                                &self.ttl_seconds,
                                &&self.last_update,
                            ];
                            ::core::fmt::Formatter::debug_struct_fields_finish(
                                f,
                                "Record",
                                names,
                                values,
                            )
                        }
                    }
                    ::sqlx::query_with::<
                        sqlx::sqlite::Sqlite,
                        _,
                    >("SELECT * FROM Feed WHERE link_code = ?", query_args)
                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                            use ::sqlx::Row as _;
                            let sqlx_query_as_id = row
                                .try_get_unchecked::<i64, _>(0usize)?;
                            let sqlx_query_as_name = row
                                .try_get_unchecked::<String, _>(1usize)?;
                            let sqlx_query_as_link_code = row
                                .try_get_unchecked::<String, _>(2usize)?;
                            let sqlx_query_as_filters = row
                                .try_get_unchecked::<Vec<u8>, _>(3usize)?;
                            let sqlx_query_as_source_link = row
                                .try_get_unchecked::<
                                    ::std::option::Option<String>,
                                    _,
                                >(4usize)?;
                            let sqlx_query_as_ttl_seconds = row
                                .try_get_unchecked::<
                                    ::std::option::Option<i64>,
                                    _,
                                >(5usize)?;
                            let sqlx_query_as_last_update = row
                                .try_get_unchecked::<
                                    ::std::option::Option<i64>,
                                    _,
                                >(6usize)?;
                            Ok(Record {
                                id: sqlx_query_as_id,
                                name: sqlx_query_as_name,
                                link_code: sqlx_query_as_link_code,
                                filters: sqlx_query_as_filters,
                                source_link: sqlx_query_as_source_link,
                                ttl_seconds: sqlx_query_as_ttl_seconds,
                                last_update: sqlx_query_as_last_update,
                            })
                        })
                }
            }
        }
            .fetch_optional(&pool)
            .await? else {
            return Ok(
                make_error_page(
                    "404 Not Found",
                    {
                        extern crate alloc;
                        extern crate maud;
                        let mut __maud_output = alloc::string::String::with_capacity(
                            56usize,
                        );
                        {
                            use ::maud::macro_private::*;
                            match ChooseRenderOrDisplay(
                                &format_args!("No feed found with link code {0:?}", code),
                            ) {
                                x => {
                                    (&&x)
                                        .implements_render_or_display()
                                        .render_to(x.0, &mut __maud_output)
                                }
                            }
                        };
                        maud::PreEscaped(__maud_output)
                    },
                    StatusCode::NOT_FOUND,
                ),
            );
        };
        Ok(
            {
                let res = ::alloc::fmt::format(format_args!("Foud {0:?}", feed));
                res
            }
                .into_response(),
        )
    }
    #[allow(warnings)]
    #[allow(unreachable_code)]
    #[doc(hidden)]
    async fn __axum_macros_check_export_into_response() {
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        async fn __axum_macros_check_export_into_response_make_value() -> ServerResult<
            Response,
        > {
            let State(pool): State<SqlitePool> = ::core::panicking::panic(
                "explicit panic",
            );
            let Path(mut code): Path<String> = ::core::panicking::panic(
                "explicit panic",
            );
            {
                if let Some(dot) = code.find('.') {
                    code.truncate(dot);
                }
                code.make_ascii_lowercase();
                let Some(feed) = {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(code);
                            let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    1usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::sqlite::Sqlite,
                                        >::size_hint(arg0),
                                );
                            query_args.add(arg0);
                            struct Record {
                                id: i64,
                                name: String,
                                link_code: String,
                                filters: Vec<u8>,
                                source_link: ::std::option::Option<String>,
                                ttl_seconds: ::std::option::Option<i64>,
                                last_update: ::std::option::Option<i64>,
                            }
                            #[automatically_derived]
                            impl ::core::fmt::Debug for Record {
                                fn fmt(
                                    &self,
                                    f: &mut ::core::fmt::Formatter,
                                ) -> ::core::fmt::Result {
                                    let names: &'static _ = &[
                                        "id",
                                        "name",
                                        "link_code",
                                        "filters",
                                        "source_link",
                                        "ttl_seconds",
                                        "last_update",
                                    ];
                                    let values: &[&dyn ::core::fmt::Debug] = &[
                                        &self.id,
                                        &self.name,
                                        &self.link_code,
                                        &self.filters,
                                        &self.source_link,
                                        &self.ttl_seconds,
                                        &&self.last_update,
                                    ];
                                    ::core::fmt::Formatter::debug_struct_fields_finish(
                                        f,
                                        "Record",
                                        names,
                                        values,
                                    )
                                }
                            }
                            ::sqlx::query_with::<
                                sqlx::sqlite::Sqlite,
                                _,
                            >("SELECT * FROM Feed WHERE link_code = ?", query_args)
                                .try_map(|row: sqlx::sqlite::SqliteRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<i64, _>(0usize)?;
                                    let sqlx_query_as_name = row
                                        .try_get_unchecked::<String, _>(1usize)?;
                                    let sqlx_query_as_link_code = row
                                        .try_get_unchecked::<String, _>(2usize)?;
                                    let sqlx_query_as_filters = row
                                        .try_get_unchecked::<Vec<u8>, _>(3usize)?;
                                    let sqlx_query_as_source_link = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(4usize)?;
                                    let sqlx_query_as_ttl_seconds = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<i64>,
                                            _,
                                        >(5usize)?;
                                    let sqlx_query_as_last_update = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<i64>,
                                            _,
                                        >(6usize)?;
                                    Ok(Record {
                                        id: sqlx_query_as_id,
                                        name: sqlx_query_as_name,
                                        link_code: sqlx_query_as_link_code,
                                        filters: sqlx_query_as_filters,
                                        source_link: sqlx_query_as_source_link,
                                        ttl_seconds: sqlx_query_as_ttl_seconds,
                                        last_update: sqlx_query_as_last_update,
                                    })
                                })
                        }
                    }
                }
                    .fetch_optional(&pool)
                    .await? else {
                    return Ok(
                        make_error_page(
                            "404 Not Found",
                            {
                                extern crate alloc;
                                extern crate maud;
                                let mut __maud_output = alloc::string::String::with_capacity(
                                    56usize,
                                );
                                {
                                    use ::maud::macro_private::*;
                                    match ChooseRenderOrDisplay(
                                        &format_args!("No feed found with link code {0:?}", code),
                                    ) {
                                        x => {
                                            (&&x)
                                                .implements_render_or_display()
                                                .render_to(x.0, &mut __maud_output)
                                        }
                                    }
                                };
                                maud::PreEscaped(__maud_output)
                            },
                            StatusCode::NOT_FOUND,
                        ),
                    );
                };
                Ok(
                    {
                        let res = ::alloc::fmt::format(format_args!("Foud {0:?}", feed));
                        res
                    }
                        .into_response(),
                )
            }
        }
        let value = __axum_macros_check_export_into_response_make_value().await;
        fn check<T>(_: T)
        where
            T: ::axum::response::IntoResponse,
        {}
        check(value);
    }
    #[allow(warnings)]
    #[allow(unreachable_code)]
    #[doc(hidden)]
    fn __axum_macros_check_export_0_from_request_check()
    where
        State<SqlitePool>: ::axum::extract::FromRequestParts<SqlitePool> + Send,
    {}
    #[allow(warnings)]
    #[allow(unreachable_code)]
    #[doc(hidden)]
    fn __axum_macros_check_export_0_from_request_call_check() {
        __axum_macros_check_export_0_from_request_check();
    }
    #[allow(warnings)]
    #[allow(unreachable_code)]
    #[doc(hidden)]
    fn __axum_macros_check_export_1_from_request_check<M>()
    where
        Path<
            String,
        >: ::axum::extract::FromRequest<SqlitePool, axum::body::Body, M> + Send,
    {}
    #[allow(warnings)]
    #[allow(unreachable_code)]
    #[doc(hidden)]
    fn __axum_macros_check_export_1_from_request_call_check() {
        __axum_macros_check_export_1_from_request_check();
    }
    #[allow(warnings)]
    #[allow(unreachable_code)]
    #[doc(hidden)]
    fn __axum_macros_check_export_future() {
        pub async fn export(
            State(pool): State<SqlitePool>,
            Path(mut code): Path<String>,
        ) -> ServerResult<Response> {
            if let Some(dot) = code.find('.') {
                code.truncate(dot);
            }
            code.make_ascii_lowercase();
            let Some(feed) = {
                {
                    #[allow(clippy::all)]
                    {
                        use ::sqlx::Arguments as _;
                        let arg0 = &(code);
                        let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::HasArguments>::Arguments::default();
                        query_args
                            .reserve(
                                1usize,
                                0
                                    + ::sqlx::encode::Encode::<
                                        sqlx::sqlite::Sqlite,
                                    >::size_hint(arg0),
                            );
                        query_args.add(arg0);
                        struct Record {
                            id: i64,
                            name: String,
                            link_code: String,
                            filters: Vec<u8>,
                            source_link: ::std::option::Option<String>,
                            ttl_seconds: ::std::option::Option<i64>,
                            last_update: ::std::option::Option<i64>,
                        }
                        #[automatically_derived]
                        impl ::core::fmt::Debug for Record {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                let names: &'static _ = &[
                                    "id",
                                    "name",
                                    "link_code",
                                    "filters",
                                    "source_link",
                                    "ttl_seconds",
                                    "last_update",
                                ];
                                let values: &[&dyn ::core::fmt::Debug] = &[
                                    &self.id,
                                    &self.name,
                                    &self.link_code,
                                    &self.filters,
                                    &self.source_link,
                                    &self.ttl_seconds,
                                    &&self.last_update,
                                ];
                                ::core::fmt::Formatter::debug_struct_fields_finish(
                                    f,
                                    "Record",
                                    names,
                                    values,
                                )
                            }
                        }
                        ::sqlx::query_with::<
                            sqlx::sqlite::Sqlite,
                            _,
                        >("SELECT * FROM Feed WHERE link_code = ?", query_args)
                            .try_map(|row: sqlx::sqlite::SqliteRow| {
                                use ::sqlx::Row as _;
                                let sqlx_query_as_id = row
                                    .try_get_unchecked::<i64, _>(0usize)?;
                                let sqlx_query_as_name = row
                                    .try_get_unchecked::<String, _>(1usize)?;
                                let sqlx_query_as_link_code = row
                                    .try_get_unchecked::<String, _>(2usize)?;
                                let sqlx_query_as_filters = row
                                    .try_get_unchecked::<Vec<u8>, _>(3usize)?;
                                let sqlx_query_as_source_link = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(4usize)?;
                                let sqlx_query_as_ttl_seconds = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<i64>,
                                        _,
                                    >(5usize)?;
                                let sqlx_query_as_last_update = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<i64>,
                                        _,
                                    >(6usize)?;
                                Ok(Record {
                                    id: sqlx_query_as_id,
                                    name: sqlx_query_as_name,
                                    link_code: sqlx_query_as_link_code,
                                    filters: sqlx_query_as_filters,
                                    source_link: sqlx_query_as_source_link,
                                    ttl_seconds: sqlx_query_as_ttl_seconds,
                                    last_update: sqlx_query_as_last_update,
                                })
                            })
                    }
                }
            }
                .fetch_optional(&pool)
                .await? else {
                return Ok(
                    make_error_page(
                        "404 Not Found",
                        {
                            extern crate alloc;
                            extern crate maud;
                            let mut __maud_output = alloc::string::String::with_capacity(
                                56usize,
                            );
                            {
                                use ::maud::macro_private::*;
                                match ChooseRenderOrDisplay(
                                    &format_args!("No feed found with link code {0:?}", code),
                                ) {
                                    x => {
                                        (&&x)
                                            .implements_render_or_display()
                                            .render_to(x.0, &mut __maud_output)
                                    }
                                }
                            };
                            maud::PreEscaped(__maud_output)
                        },
                        StatusCode::NOT_FOUND,
                    ),
                );
            };
            Ok(
                {
                    let res = ::alloc::fmt::format(format_args!("Foud {0:?}", feed));
                    res
                }
                    .into_response(),
            )
        }
        let future = export(
            ::core::panicking::panic("explicit panic"),
            ::core::panicking::panic("explicit panic"),
        );
        fn check<T>(_: T)
        where
            T: ::std::future::Future + Send,
        {}
        check(future);
    }
}
mod presentation {
    mod htmx {
        use std::convert::Infallible;
        use async_trait::async_trait;
        use axum::{extract::FromRequest, http::{HeaderMap, HeaderValue, Request}};
        pub struct HxWrap {
            is_htmx: bool,
        }
        impl HxWrap {
            pub fn wrap<T>(&self, inner: T, wrap: impl FnOnce(T) -> T) -> T {
                if self.is_htmx { inner } else { wrap(inner) }
            }
        }
        impl<S, B> FromRequest<S, B> for HxWrap
        where
            B: Send + 'static,
            S: Send + Sync,
        {
            type Rejection = Infallible;
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn from_request<'life0, 'async_trait>(
                req: Request<B>,
                state: &'life0 S,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<Self, Self::Rejection>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret)
                        = ::core::option::Option::None::<Result<Self, Self::Rejection>> {
                        return __ret;
                    }
                    let req = req;
                    let __ret: Result<Self, Self::Rejection> = {
                        let headers = HeaderMap::from_request(req, state).await?;
                        let is_htmx = headers.get("HX-Request")
                            == Some(&HeaderValue::from_static("true"));
                        Ok(Self { is_htmx })
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    mod icon {
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
            {
                extern crate alloc;
                extern crate maud;
                let mut __maud_output = alloc::string::String::with_capacity(404usize);
                __maud_output.push_str("<svg class=\"icon\"");
                if let Some(inner_value) = (data.style) {
                    __maud_output.push_str(" style=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                if let Some(inner_value) = (data.x) {
                    __maud_output.push_str(" x=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                if let Some(inner_value) = (data.y) {
                    __maud_output.push_str(" y=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                if let Some(inner_value) = (data.view_box) {
                    __maud_output.push_str(" viewBox=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                if let Some(inner_value) = (data.stroke_linecap) {
                    __maud_output.push_str(" stroke-linecap=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                if let Some(inner_value) = (data.stroke_linejoin) {
                    __maud_output.push_str(" stroke-linejoin=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                if let Some(inner_value) = (data.stroke_width) {
                    __maud_output.push_str(" stroke-width=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                __maud_output.push_str(" stroke=\"");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&data.stroke.unwrap_or("currentColor")) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("\" fill=\"");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&data.fill.unwrap_or("currentColor")) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("\" role=\"graphics-symbol\">");
                if let Some(alt) = alt {
                    __maud_output.push_str("<title>");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&alt) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("</title>");
                }
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&PreEscaped(data.data)) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("</svg>");
                maud::PreEscaped(__maud_output)
            }
        }
    }
    mod layout {
        use maud::{html, Markup, DOCTYPE};
        pub fn base_layout(content: Markup) -> Markup {
            {
                extern crate alloc;
                extern crate maud;
                let mut __maud_output = alloc::string::String::with_capacity(541usize);
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&DOCTYPE) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output
                    .push_str(
                        "<html lang=\"en\"><head><meta charset=\"utf-8\"><meta http-equiv=\"x-ua-compatible\" content=\"ie=edge\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>iCal Manager</title><link rel=\"stylesheet\" href=\"/css/primitive.css\"><link rel=\"stylesheet\" href=\"/css/main.css\"><link rel=\"icon\" href=\"/images/favicon.png\"></head><body>",
                    );
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&content) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output
                    .push_str(
                        "<script src=\"/js/htmx.min.js\"></script><script src=\"/js/main.js\"></script></body></html>",
                    );
                maud::PreEscaped(__maud_output)
            }
        }
        pub fn layout(content: Markup) -> Markup {
            base_layout({
                extern crate alloc;
                extern crate maud;
                let mut __maud_output = alloc::string::String::with_capacity(75usize);
                __maud_output
                    .push_str(
                        "<div class=\"medium-container\"><h1><a href=\"/\">iCal Manager</a></h1>",
                    );
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&content) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("</div>");
                maud::PreEscaped(__maud_output)
            })
        }
    }
    mod fragments {
        use std::{
            fmt::{self, Display},
            time::Duration,
        };
        use anyhow::Context;
        use axum::response::IntoResponse;
        use icondata::LuIcon;
        use maud::{html, Markup};
        use sqlx::SqlitePool;
        use strum::{EnumString, IntoStaticStr};
        use super::{error::ServerResult, icon::icon};
        fn feed_row(id: i64, name: &str, link_code: &str) -> anyhow::Result<Markup> {
            Ok({
                extern crate alloc;
                extern crate maud;
                let mut __maud_output = alloc::string::String::with_capacity(516usize);
                __maud_output.push_str("<tr><td>");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&name) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("</td><td>");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&feed_status_loader(id)) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output
                    .push_str(
                        "</td><td class=\"actions\"><a class=\"button small-button round-button\" href=\"",
                    );
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&format_args!("/feed/{0}/edit", id)) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("\">");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&icon(LuIcon::LuEdit)) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output
                    .push_str(
                        " Edit</a><button class=\"small-button muted-button round-button\" onclick=\"copyFeedLink(this);\">",
                    );
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&icon(LuIcon::LuCopy)) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output
                    .push_str(
                        " Copy Public Link</button><input type=\"text\" class=\"copy-source\" data-partial-copy-uri=\"",
                    );
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(
                        &format_args!("/export/{0}.ical", link_code),
                    ) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("\" readonly></input></td></tr>");
                maud::PreEscaped(__maud_output)
            })
        }
        pub async fn feed_table(pool: &SqlitePool) -> anyhow::Result<Markup> {
            let feeds = {
                {
                    #[allow(clippy::all)]
                    {
                        use ::sqlx::Arguments as _;
                        let query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::HasArguments>::Arguments::default();
                        struct Record {
                            id: i64,
                            name: String,
                            link_code: String,
                        }
                        #[automatically_derived]
                        impl ::core::fmt::Debug for Record {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                ::core::fmt::Formatter::debug_struct_field3_finish(
                                    f,
                                    "Record",
                                    "id",
                                    &self.id,
                                    "name",
                                    &self.name,
                                    "link_code",
                                    &&self.link_code,
                                )
                            }
                        }
                        ::sqlx::query_with::<
                            sqlx::sqlite::Sqlite,
                            _,
                        >("SELECT id, name, link_code FROM Feed", query_args)
                            .try_map(|row: sqlx::sqlite::SqliteRow| {
                                use ::sqlx::Row as _;
                                let sqlx_query_as_id = row
                                    .try_get_unchecked::<i64, _>(0usize)?;
                                let sqlx_query_as_name = row
                                    .try_get_unchecked::<String, _>(1usize)?;
                                let sqlx_query_as_link_code = row
                                    .try_get_unchecked::<String, _>(2usize)?;
                                Ok(Record {
                                    id: sqlx_query_as_id,
                                    name: sqlx_query_as_name,
                                    link_code: sqlx_query_as_link_code,
                                })
                            })
                    }
                }
            }
                .fetch_all(pool)
                .await
                .context("Error fetching feed list")?;
            if feeds.is_empty() {
                return Ok({
                    extern crate alloc;
                    extern crate maud;
                    let mut __maud_output = alloc::string::String::with_capacity(
                        36usize,
                    );
                    __maud_output
                        .push_str("<p class=\"text-center\">No feeds found</p>");
                    maud::PreEscaped(__maud_output)
                });
            }
            Ok({
                extern crate alloc;
                extern crate maud;
                let mut __maud_output = alloc::string::String::with_capacity(223usize);
                __maud_output
                    .push_str(
                        "<table class=\"striped-table\" id=\"feeds-table\"><thead><tr><th>Name</th><th>Status</th><th>Actions</th></tr></thead><tbody>",
                    );
                for feed in feeds {
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(
                            &feed_row(feed.id, &feed.name, &feed.link_code)?,
                        ) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                }
                __maud_output.push_str("</tbody></table>");
                maud::PreEscaped(__maud_output)
            })
        }
        enum FeedStatus {
            Loading,
            Error,
            Warn,
            Ok,
        }
        fn feed_status_id(id: impl Display) -> String {
            {
                let res = ::alloc::fmt::format(format_args!("feed-status-{0}", id));
                res
            }
        }
        fn feed_status_base(
            status: FeedStatus,
            id: i64,
            text: &str,
            hx_get: Option<fmt::Arguments>,
        ) -> Markup {
            let type_class = match status {
                FeedStatus::Loading => "status-loading",
                FeedStatus::Error => "status-error",
                FeedStatus::Warn => "status-warn",
                FeedStatus::Ok => "status-ok",
            };
            {
                extern crate alloc;
                extern crate maud;
                let mut __maud_output = alloc::string::String::with_capacity(163usize);
                __maud_output.push_str("<div class=\"status ");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&type_class) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("\" id=\"");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&feed_status_id(id)) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("\"");
                if let Some(inner_value) = (hx_get) {
                    __maud_output.push_str(" hx-get=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                if let Some(inner_value) = (hx_get.and(Some("outerHTML"))) {
                    __maud_output.push_str(" hx-swap=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                if let Some(inner_value) = (hx_get.and(Some("load"))) {
                    __maud_output.push_str(" hx-trigger=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                __maud_output.push_str(">");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&text) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("</div>");
                maud::PreEscaped(__maud_output)
            }
        }
        fn feed_status_loader(id: i64) -> Markup {
            feed_status_base(
                FeedStatus::Loading,
                id,
                "Loading...",
                Some(format_args!("/feed/{0}/status", id)),
            )
        }
        fn feed_status_result(status: FeedStatus, id: i64, text: &str) -> Markup {
            feed_status_base(status, id, text, None)
        }
        pub async fn feed_status(pool: &SqlitePool, id: i64) -> anyhow::Result<Markup> {
            let Some(feed) = {
                {
                    #[allow(clippy::all)]
                    {
                        use ::sqlx::Arguments as _;
                        let arg0 = &(id);
                        let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::HasArguments>::Arguments::default();
                        query_args
                            .reserve(
                                1usize,
                                0
                                    + ::sqlx::encode::Encode::<
                                        sqlx::sqlite::Sqlite,
                                    >::size_hint(arg0),
                            );
                        query_args.add(arg0);
                        struct Record {
                            id: i64,
                        }
                        #[automatically_derived]
                        impl ::core::fmt::Debug for Record {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                ::core::fmt::Formatter::debug_struct_field1_finish(
                                    f,
                                    "Record",
                                    "id",
                                    &&self.id,
                                )
                            }
                        }
                        ::sqlx::query_with::<
                            sqlx::sqlite::Sqlite,
                            _,
                        >("SELECT id FROM Feed WHERE id = ?", query_args)
                            .try_map(|row: sqlx::sqlite::SqliteRow| {
                                use ::sqlx::Row as _;
                                let sqlx_query_as_id = row
                                    .try_get_unchecked::<i64, _>(0usize)?;
                                Ok(Record { id: sqlx_query_as_id })
                            })
                    }
                }
            }
                .fetch_optional(pool)
                .await
                .context("Error fetching feed list")? else {
                return Ok(feed_status_result(FeedStatus::Error, id, "Not Found"));
            };
            return Ok(feed_status_result(FeedStatus::Ok, feed.id, "Found"));
        }
        #[strum(serialize_all = "kebab-case")]
        pub enum FeedType {
            Upload,
            Link,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for FeedType {
            #[inline]
            fn clone(&self) -> FeedType {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for FeedType {}
        #[automatically_derived]
        impl ::core::fmt::Debug for FeedType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        FeedType::Upload => "Upload",
                        FeedType::Link => "Link",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for FeedType {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for FeedType {
            #[inline]
            fn eq(&self, other: &FeedType) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[allow(clippy::use_self)]
        impl ::core::str::FromStr for FeedType {
            type Err = ::strum::ParseError;
            fn from_str(
                s: &str,
            ) -> ::core::result::Result<FeedType, <Self as ::core::str::FromStr>::Err> {
                ::core::result::Result::Ok(
                    match s {
                        "upload" => FeedType::Upload,
                        "link" => FeedType::Link,
                        _ => {
                            return ::core::result::Result::Err(
                                ::strum::ParseError::VariantNotFound,
                            );
                        }
                    },
                )
            }
        }
        #[allow(clippy::use_self)]
        impl ::core::convert::TryFrom<&str> for FeedType {
            type Error = ::strum::ParseError;
            fn try_from(
                s: &str,
            ) -> ::core::result::Result<
                FeedType,
                <Self as ::core::convert::TryFrom<&str>>::Error,
            > {
                ::core::str::FromStr::from_str(s)
            }
        }
        impl ::core::convert::From<FeedType> for &'static str {
            fn from(x: FeedType) -> &'static str {
                match x {
                    FeedType::Upload => "upload",
                    FeedType::Link => "link",
                }
            }
        }
        impl<'_derivative_strum> ::core::convert::From<&'_derivative_strum FeedType>
        for &'static str {
            fn from(x: &'_derivative_strum FeedType) -> &'static str {
                match *x {
                    FeedType::Upload => "upload",
                    FeedType::Link => "link",
                }
            }
        }
        pub struct FormValues {
            pub name: String,
            pub feed_type: FeedType,
            pub link: String,
            pub ttl: Option<Duration>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for FormValues {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "FormValues",
                    "name",
                    &self.name,
                    "feed_type",
                    &self.feed_type,
                    "link",
                    &self.link,
                    "ttl",
                    &&self.ttl,
                )
            }
        }
        fn select_options(
            selected_value: Option<&str>,
            options: &[(&str, &str)],
        ) -> Markup {
            {
                extern crate alloc;
                extern crate maud;
                let mut __maud_output = alloc::string::String::with_capacity(118usize);
                for (value, label) in options {
                    __maud_output.push_str("<option value=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                    if (selected_value == Some(value)) {
                        __maud_output.push_str(" selected");
                    }
                    __maud_output.push_str(">");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&label) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("</option>");
                }
                maud::PreEscaped(__maud_output)
            }
        }
        const DURATION_6_HOURS: Duration = Duration::from_secs(6 * 60 * 60);
        pub fn feed_form(values: Option<&FormValues>) -> Markup {
            let (name, feed_type, link, ttl) = match values {
                Some(FormValues { name, feed_type, link, ttl }) => {
                    (Some(name.as_str()), Some(feed_type), Some(link.as_str()), *ttl)
                }
                None => (None, None, None, None),
            };
            let ttl = humantime::format_duration(ttl.unwrap_or(DURATION_6_HOURS));
            {
                extern crate alloc;
                extern crate maud;
                let mut __maud_output = alloc::string::String::with_capacity(989usize);
                __maud_output
                    .push_str(
                        "<form id=\"form-form\"><h3>Source</h3><label for=\"name\">Name</label><input id=\"name\" type=\"text\"",
                    );
                if let Some(inner_value) = (name) {
                    __maud_output.push_str(" value=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                __maud_output
                    .push_str(
                        "><label for=\"type\">Type</label><select id=\"type\" onchange=\"updateFormForm();\"",
                    );
                if let Some(inner_value) = (feed_type.map(<&str>::from)) {
                    __maud_output.push_str(" value=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                __maud_output.push_str(">");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(
                        &select_options(
                            feed_type.map(<&str>::from),
                            &[("link", "Fetch URL"), ("upload", "File Upload")],
                        ),
                    ) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output
                    .push_str(
                        "</select><div class=\"hide\" id=\"section-link\"><label for=\"link\">Link</label><input id=\"link\" type=\"text\" placeholder=\"https://example.com/feed.ical\"",
                    );
                if let Some(inner_value) = (link) {
                    __maud_output.push_str(" value=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&inner_value) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                __maud_output
                    .push_str(
                        "><label for=\"ttl\">Minimum Update Period</label><input id=\"ttl\" type=\"text\" value=\"",
                    );
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&ttl) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output
                    .push_str(
                        "\"><p>Enter a value like 1 day, 15min, 3h, etc...</p></div><div class=\"hide\" id=\"section-upload\"><label for=\"upload\">Upload iCal</label><input id=\"upload\" type=\"file\"></div><h3>Filters</h3><label for=\"filter-cr\"><input id=\"filter-cr\" type=\"checkbox\"> Remove Carriage Returns (<code>\\r</code>)</label><input type=\"submit\"></form>",
                    );
                maud::PreEscaped(__maud_output)
            }
        }
    }
    pub mod pages {
        use std::time::Duration;
        use axum::{
            extract::{Path, State},
            response::{IntoResponse, Response},
        };
        use icondata::LuIcon;
        use maud::{html, Markup};
        use sqlx::SqlitePool;
        use super::{
            error::{make_404, ServerResult},
            fragments::{self, feed_form, FeedType, FormValues},
            htmx::HxWrap, icon::icon_alt, layout::layout,
        };
        pub async fn feed_status(
            State(pool): State<SqlitePool>,
            Path((id,)): Path<(i64,)>,
        ) -> ServerResult<impl IntoResponse> {
            Ok(fragments::feed_status(&pool, id).await?.into_response())
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        async fn __axum_macros_check_feed_status_into_response() {
            #[allow(warnings)]
            #[allow(unreachable_code)]
            #[doc(hidden)]
            async fn __axum_macros_check_feed_status_into_response_make_value() -> ServerResult<
                impl IntoResponse,
            > {
                let State(pool): State<SqlitePool> = ::core::panicking::panic(
                    "explicit panic",
                );
                let Path((id,)): Path<(i64,)> = ::core::panicking::panic(
                    "explicit panic",
                );
                { Ok(fragments::feed_status(&pool, id).await?.into_response()) }
            }
            let value = __axum_macros_check_feed_status_into_response_make_value().await;
            fn check<T>(_: T)
            where
                T: ::axum::response::IntoResponse,
            {}
            check(value);
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_feed_status_0_from_request_check()
        where
            State<SqlitePool>: ::axum::extract::FromRequestParts<SqlitePool> + Send,
        {}
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_feed_status_0_from_request_call_check() {
            __axum_macros_check_feed_status_0_from_request_check();
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_feed_status_1_from_request_check<M>()
        where
            Path<
                (i64,),
            >: ::axum::extract::FromRequest<SqlitePool, axum::body::Body, M> + Send,
        {}
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_feed_status_1_from_request_call_check() {
            __axum_macros_check_feed_status_1_from_request_check();
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_feed_status_future() {
            pub async fn feed_status(
                State(pool): State<SqlitePool>,
                Path((id,)): Path<(i64,)>,
            ) -> ServerResult<impl IntoResponse> {
                Ok(fragments::feed_status(&pool, id).await?.into_response())
            }
            let future = feed_status(
                ::core::panicking::panic("explicit panic"),
                ::core::panicking::panic("explicit panic"),
            );
            fn check<T>(_: T)
            where
                T: ::std::future::Future + Send,
            {}
            check(future);
        }
        pub async fn root(
            State(pool): State<SqlitePool>,
            hx_wrap: HxWrap,
        ) -> ServerResult<Markup> {
            let table = fragments::feed_table(&pool).await?;
            Ok(
                hx_wrap
                    .wrap(
                        table,
                        |inner| {
                            layout({
                                extern crate alloc;
                                extern crate maud;
                                let mut __maud_output = alloc::string::String::with_capacity(
                                    264usize,
                                );
                                __maud_output
                                    .push_str(
                                        "<h2>Feeds <button class=\"small-button muted-button round-button\" id=\"feed-spinner\" hx-get=\"/\" hx-target=\"#feeds-table\" hx-indicator><div class=\"htmx-spinner\">",
                                    );
                                {
                                    use ::maud::macro_private::*;
                                    match ChooseRenderOrDisplay(
                                        &icon_alt(LuIcon::LuRefreshCw, "Refresh Feeds"),
                                    ) {
                                        x => {
                                            (&&x)
                                                .implements_render_or_display()
                                                .render_to(x.0, &mut __maud_output)
                                        }
                                    }
                                };
                                __maud_output.push_str("</div></button></h2>");
                                {
                                    use ::maud::macro_private::*;
                                    match ChooseRenderOrDisplay(&inner) {
                                        x => {
                                            (&&x)
                                                .implements_render_or_display()
                                                .render_to(x.0, &mut __maud_output)
                                        }
                                    }
                                };
                                maud::PreEscaped(__maud_output)
                            })
                        },
                    ),
            )
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        async fn __axum_macros_check_root_into_response() {
            #[allow(warnings)]
            #[allow(unreachable_code)]
            #[doc(hidden)]
            async fn __axum_macros_check_root_into_response_make_value() -> ServerResult<
                Markup,
            > {
                let State(pool): State<SqlitePool> = ::core::panicking::panic(
                    "explicit panic",
                );
                let hx_wrap: HxWrap = ::core::panicking::panic("explicit panic");
                {
                    let table = fragments::feed_table(&pool).await?;
                    Ok(
                        hx_wrap
                            .wrap(
                                table,
                                |inner| {
                                    layout({
                                        extern crate alloc;
                                        extern crate maud;
                                        let mut __maud_output = alloc::string::String::with_capacity(
                                            264usize,
                                        );
                                        __maud_output
                                            .push_str(
                                                "<h2>Feeds <button class=\"small-button muted-button round-button\" id=\"feed-spinner\" hx-get=\"/\" hx-target=\"#feeds-table\" hx-indicator><div class=\"htmx-spinner\">",
                                            );
                                        {
                                            use ::maud::macro_private::*;
                                            match ChooseRenderOrDisplay(
                                                &icon_alt(LuIcon::LuRefreshCw, "Refresh Feeds"),
                                            ) {
                                                x => {
                                                    (&&x)
                                                        .implements_render_or_display()
                                                        .render_to(x.0, &mut __maud_output)
                                                }
                                            }
                                        };
                                        __maud_output.push_str("</div></button></h2>");
                                        {
                                            use ::maud::macro_private::*;
                                            match ChooseRenderOrDisplay(&inner) {
                                                x => {
                                                    (&&x)
                                                        .implements_render_or_display()
                                                        .render_to(x.0, &mut __maud_output)
                                                }
                                            }
                                        };
                                        maud::PreEscaped(__maud_output)
                                    })
                                },
                            ),
                    )
                }
            }
            let value = __axum_macros_check_root_into_response_make_value().await;
            fn check<T>(_: T)
            where
                T: ::axum::response::IntoResponse,
            {}
            check(value);
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_root_0_from_request_check()
        where
            State<SqlitePool>: ::axum::extract::FromRequestParts<SqlitePool> + Send,
        {}
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_root_0_from_request_call_check() {
            __axum_macros_check_root_0_from_request_check();
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_root_1_from_request_check<M>()
        where
            HxWrap: ::axum::extract::FromRequest<SqlitePool, axum::body::Body, M> + Send,
        {}
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_root_1_from_request_call_check() {
            __axum_macros_check_root_1_from_request_check();
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_root_future() {
            pub async fn root(
                State(pool): State<SqlitePool>,
                hx_wrap: HxWrap,
            ) -> ServerResult<Markup> {
                let table = fragments::feed_table(&pool).await?;
                Ok(
                    hx_wrap
                        .wrap(
                            table,
                            |inner| {
                                layout({
                                    extern crate alloc;
                                    extern crate maud;
                                    let mut __maud_output = alloc::string::String::with_capacity(
                                        264usize,
                                    );
                                    __maud_output
                                        .push_str(
                                            "<h2>Feeds <button class=\"small-button muted-button round-button\" id=\"feed-spinner\" hx-get=\"/\" hx-target=\"#feeds-table\" hx-indicator><div class=\"htmx-spinner\">",
                                        );
                                    {
                                        use ::maud::macro_private::*;
                                        match ChooseRenderOrDisplay(
                                            &icon_alt(LuIcon::LuRefreshCw, "Refresh Feeds"),
                                        ) {
                                            x => {
                                                (&&x)
                                                    .implements_render_or_display()
                                                    .render_to(x.0, &mut __maud_output)
                                            }
                                        }
                                    };
                                    __maud_output.push_str("</div></button></h2>");
                                    {
                                        use ::maud::macro_private::*;
                                        match ChooseRenderOrDisplay(&inner) {
                                            x => {
                                                (&&x)
                                                    .implements_render_or_display()
                                                    .render_to(x.0, &mut __maud_output)
                                            }
                                        }
                                    };
                                    maud::PreEscaped(__maud_output)
                                })
                            },
                        ),
                )
            }
            let future = root(
                ::core::panicking::panic("explicit panic"),
                ::core::panicking::panic("explicit panic"),
            );
            fn check<T>(_: T)
            where
                T: ::std::future::Future + Send,
            {}
            check(future);
        }
        pub async fn feed_edit(
            State(pool): State<SqlitePool>,
            Path((id,)): Path<(i64,)>,
        ) -> ServerResult<Response> {
            let Some(feed) = {
                {
                    #[allow(clippy::all)]
                    {
                        use ::sqlx::Arguments as _;
                        let arg0 = &(id);
                        let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::HasArguments>::Arguments::default();
                        query_args
                            .reserve(
                                1usize,
                                0
                                    + ::sqlx::encode::Encode::<
                                        sqlx::sqlite::Sqlite,
                                    >::size_hint(arg0),
                            );
                        query_args.add(arg0);
                        struct Record {
                            name: String,
                            source_link: ::std::option::Option<String>,
                            ttl_seconds: ::std::option::Option<i64>,
                        }
                        #[automatically_derived]
                        impl ::core::fmt::Debug for Record {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                ::core::fmt::Formatter::debug_struct_field3_finish(
                                    f,
                                    "Record",
                                    "name",
                                    &self.name,
                                    "source_link",
                                    &self.source_link,
                                    "ttl_seconds",
                                    &&self.ttl_seconds,
                                )
                            }
                        }
                        ::sqlx::query_with::<
                            sqlx::sqlite::Sqlite,
                            _,
                        >(
                                "SELECT name, source_link, ttl_seconds FROM Feed WHERE id = ?",
                                query_args,
                            )
                            .try_map(|row: sqlx::sqlite::SqliteRow| {
                                use ::sqlx::Row as _;
                                let sqlx_query_as_name = row
                                    .try_get_unchecked::<String, _>(0usize)?;
                                let sqlx_query_as_source_link = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(1usize)?;
                                let sqlx_query_as_ttl_seconds = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<i64>,
                                        _,
                                    >(2usize)?;
                                Ok(Record {
                                    name: sqlx_query_as_name,
                                    source_link: sqlx_query_as_source_link,
                                    ttl_seconds: sqlx_query_as_ttl_seconds,
                                })
                            })
                    }
                }
            }
                .fetch_optional(&pool)
                .await? else {
                return Ok(make_404());
            };
            let values = FormValues {
                name: feed.name,
                feed_type: match feed.source_link {
                    Some(_) => FeedType::Link,
                    None => FeedType::Upload,
                },
                link: feed.source_link.unwrap_or_else(String::new),
                ttl: feed.ttl_seconds.map(|seconds| Duration::from_secs(seconds as u64)),
            };
            Ok(
                layout({
                        extern crate alloc;
                        extern crate maud;
                        let mut __maud_output = alloc::string::String::with_capacity(
                            46usize,
                        );
                        __maud_output.push_str("<h2>Edit Feed</h2>");
                        {
                            use ::maud::macro_private::*;
                            match ChooseRenderOrDisplay(&feed_form(Some(&values))) {
                                x => {
                                    (&&x)
                                        .implements_render_or_display()
                                        .render_to(x.0, &mut __maud_output)
                                }
                            }
                        };
                        maud::PreEscaped(__maud_output)
                    })
                    .into_response(),
            )
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        async fn __axum_macros_check_feed_edit_into_response() {
            #[allow(warnings)]
            #[allow(unreachable_code)]
            #[doc(hidden)]
            async fn __axum_macros_check_feed_edit_into_response_make_value() -> ServerResult<
                Response,
            > {
                let State(pool): State<SqlitePool> = ::core::panicking::panic(
                    "explicit panic",
                );
                let Path((id,)): Path<(i64,)> = ::core::panicking::panic(
                    "explicit panic",
                );
                {
                    let Some(feed) = {
                        {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(id);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::HasArguments>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                query_args.add(arg0);
                                struct Record {
                                    name: String,
                                    source_link: ::std::option::Option<String>,
                                    ttl_seconds: ::std::option::Option<i64>,
                                }
                                #[automatically_derived]
                                impl ::core::fmt::Debug for Record {
                                    fn fmt(
                                        &self,
                                        f: &mut ::core::fmt::Formatter,
                                    ) -> ::core::fmt::Result {
                                        ::core::fmt::Formatter::debug_struct_field3_finish(
                                            f,
                                            "Record",
                                            "name",
                                            &self.name,
                                            "source_link",
                                            &self.source_link,
                                            "ttl_seconds",
                                            &&self.ttl_seconds,
                                        )
                                    }
                                }
                                ::sqlx::query_with::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >(
                                        "SELECT name, source_link, ttl_seconds FROM Feed WHERE id = ?",
                                        query_args,
                                    )
                                    .try_map(|row: sqlx::sqlite::SqliteRow| {
                                        use ::sqlx::Row as _;
                                        let sqlx_query_as_name = row
                                            .try_get_unchecked::<String, _>(0usize)?;
                                        let sqlx_query_as_source_link = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(1usize)?;
                                        let sqlx_query_as_ttl_seconds = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<i64>,
                                                _,
                                            >(2usize)?;
                                        Ok(Record {
                                            name: sqlx_query_as_name,
                                            source_link: sqlx_query_as_source_link,
                                            ttl_seconds: sqlx_query_as_ttl_seconds,
                                        })
                                    })
                            }
                        }
                    }
                        .fetch_optional(&pool)
                        .await? else {
                        return Ok(make_404());
                    };
                    let values = FormValues {
                        name: feed.name,
                        feed_type: match feed.source_link {
                            Some(_) => FeedType::Link,
                            None => FeedType::Upload,
                        },
                        link: feed.source_link.unwrap_or_else(String::new),
                        ttl: feed
                            .ttl_seconds
                            .map(|seconds| Duration::from_secs(seconds as u64)),
                    };
                    Ok(
                        layout({
                                extern crate alloc;
                                extern crate maud;
                                let mut __maud_output = alloc::string::String::with_capacity(
                                    46usize,
                                );
                                __maud_output.push_str("<h2>Edit Feed</h2>");
                                {
                                    use ::maud::macro_private::*;
                                    match ChooseRenderOrDisplay(&feed_form(Some(&values))) {
                                        x => {
                                            (&&x)
                                                .implements_render_or_display()
                                                .render_to(x.0, &mut __maud_output)
                                        }
                                    }
                                };
                                maud::PreEscaped(__maud_output)
                            })
                            .into_response(),
                    )
                }
            }
            let value = __axum_macros_check_feed_edit_into_response_make_value().await;
            fn check<T>(_: T)
            where
                T: ::axum::response::IntoResponse,
            {}
            check(value);
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_feed_edit_0_from_request_check()
        where
            State<SqlitePool>: ::axum::extract::FromRequestParts<SqlitePool> + Send,
        {}
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_feed_edit_0_from_request_call_check() {
            __axum_macros_check_feed_edit_0_from_request_check();
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_feed_edit_1_from_request_check<M>()
        where
            Path<
                (i64,),
            >: ::axum::extract::FromRequest<SqlitePool, axum::body::Body, M> + Send,
        {}
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_feed_edit_1_from_request_call_check() {
            __axum_macros_check_feed_edit_1_from_request_check();
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_feed_edit_future() {
            pub async fn feed_edit(
                State(pool): State<SqlitePool>,
                Path((id,)): Path<(i64,)>,
            ) -> ServerResult<Response> {
                let Some(feed) = {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(id);
                            let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    1usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::sqlite::Sqlite,
                                        >::size_hint(arg0),
                                );
                            query_args.add(arg0);
                            struct Record {
                                name: String,
                                source_link: ::std::option::Option<String>,
                                ttl_seconds: ::std::option::Option<i64>,
                            }
                            #[automatically_derived]
                            impl ::core::fmt::Debug for Record {
                                fn fmt(
                                    &self,
                                    f: &mut ::core::fmt::Formatter,
                                ) -> ::core::fmt::Result {
                                    ::core::fmt::Formatter::debug_struct_field3_finish(
                                        f,
                                        "Record",
                                        "name",
                                        &self.name,
                                        "source_link",
                                        &self.source_link,
                                        "ttl_seconds",
                                        &&self.ttl_seconds,
                                    )
                                }
                            }
                            ::sqlx::query_with::<
                                sqlx::sqlite::Sqlite,
                                _,
                            >(
                                    "SELECT name, source_link, ttl_seconds FROM Feed WHERE id = ?",
                                    query_args,
                                )
                                .try_map(|row: sqlx::sqlite::SqliteRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_name = row
                                        .try_get_unchecked::<String, _>(0usize)?;
                                    let sqlx_query_as_source_link = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(1usize)?;
                                    let sqlx_query_as_ttl_seconds = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<i64>,
                                            _,
                                        >(2usize)?;
                                    Ok(Record {
                                        name: sqlx_query_as_name,
                                        source_link: sqlx_query_as_source_link,
                                        ttl_seconds: sqlx_query_as_ttl_seconds,
                                    })
                                })
                        }
                    }
                }
                    .fetch_optional(&pool)
                    .await? else {
                    return Ok(make_404());
                };
                let values = FormValues {
                    name: feed.name,
                    feed_type: match feed.source_link {
                        Some(_) => FeedType::Link,
                        None => FeedType::Upload,
                    },
                    link: feed.source_link.unwrap_or_else(String::new),
                    ttl: feed
                        .ttl_seconds
                        .map(|seconds| Duration::from_secs(seconds as u64)),
                };
                Ok(
                    layout({
                            extern crate alloc;
                            extern crate maud;
                            let mut __maud_output = alloc::string::String::with_capacity(
                                46usize,
                            );
                            __maud_output.push_str("<h2>Edit Feed</h2>");
                            {
                                use ::maud::macro_private::*;
                                match ChooseRenderOrDisplay(&feed_form(Some(&values))) {
                                    x => {
                                        (&&x)
                                            .implements_render_or_display()
                                            .render_to(x.0, &mut __maud_output)
                                    }
                                }
                            };
                            maud::PreEscaped(__maud_output)
                        })
                        .into_response(),
                )
            }
            let future = feed_edit(
                ::core::panicking::panic("explicit panic"),
                ::core::panicking::panic("explicit panic"),
            );
            fn check<T>(_: T)
            where
                T: ::std::future::Future + Send,
            {}
            check(future);
        }
    }
    pub mod error {
        use axum::{http::StatusCode, response::{IntoResponse, Response}};
        use maud::{html, Markup};
        use super::layout::base_layout;
        pub type ServerResult<T> = Result<T, InternalServerError>;
        pub struct InternalServerError(pub anyhow::Error);
        impl<T> From<T> for InternalServerError
        where
            anyhow::Error: From<T>,
        {
            fn from(value: T) -> Self {
                Self(value.into())
            }
        }
        impl IntoResponse for InternalServerError {
            fn into_response(self) -> axum::response::Response {
                let content = {
                    extern crate alloc;
                    extern crate maud;
                    let mut __maud_output = alloc::string::String::with_capacity(
                        48usize,
                    );
                    __maud_output.push_str("<pre><code>");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&format_args!("{0:?}", self.0)) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("</code></pre>");
                    maud::PreEscaped(__maud_output)
                };
                make_error_page(
                    "500 Internal Server Error",
                    content,
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }
        }
        pub fn make_error_page(
            title: &str,
            content: Markup,
            status: StatusCode,
        ) -> Response {
            let html = base_layout({
                extern crate alloc;
                extern crate maud;
                let mut __maud_output = alloc::string::String::with_capacity(51usize);
                __maud_output.push_str("<div class=\"medium-container\"><h1>");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&title) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("</h1>");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&content) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("</div>");
                maud::PreEscaped(__maud_output)
            });
            (status, html).into_response()
        }
        pub fn make_404() -> Response {
            make_error_page(
                "404 Not Found",
                {
                    extern crate alloc;
                    extern crate maud;
                    let mut __maud_output = alloc::string::String::with_capacity(
                        26usize,
                    );
                    __maud_output.push_str("<a href=\"/\">Go Home</a>");
                    maud::PreEscaped(__maud_output)
                },
                StatusCode::NOT_FOUND,
            )
        }
    }
}
mod data {
    use std::{fmt::Display, str::FromStr};
    use anyhow::Context;
    use serde::{Deserialize, Serialize};
    use sqlx::{
        sqlite::{SqliteTypeInfo, SqliteValueRef},
        Decode, Encode, FromRow, Sqlite, Type,
    };
    enum Filter {
        RemoveCarraigeReturn,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Filter {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "RemoveCarraigeReturn")
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Filter {
        #[inline]
        fn clone(&self) -> Filter {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Filter {}
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Filter {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Filter::RemoveCarraigeReturn => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Filter",
                            0u32,
                            "RemoveCarraigeReturn",
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Filter {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "RemoveCarraigeReturn" => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"RemoveCarraigeReturn" => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Filter>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Filter;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum Filter",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Filter::RemoveCarraigeReturn)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["RemoveCarraigeReturn"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Filter",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Filter>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    struct Filters(pub Vec<Filter>);
    #[automatically_derived]
    impl ::core::fmt::Debug for Filters {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Filters", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Filters {
        #[inline]
        fn clone(&self) -> Filters {
            Filters(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Filters {
        #[inline]
        fn default() -> Filters {
            Filters(::core::default::Default::default())
        }
    }
    impl<'a> Encode<'a, Sqlite> for Filters {
        fn encode_by_ref(
            &self,
            buf: &mut <Sqlite as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
        ) -> sqlx::encode::IsNull {
            let data = serde_json::to_vec(&self.0).expect("Error serializing filters");
            data.encode(buf)
        }
    }
    impl<'a> Decode<'a, Sqlite> for Filters {
        fn decode(value: SqliteValueRef) -> Result<Self, sqlx::error::BoxDynError> {
            let data = <&[u8]>::decode(value)?;
            Ok(
                Filters(
                    serde_json::from_slice(data).context("Error deserializing filters")?,
                ),
            )
        }
    }
    impl Type<Sqlite> for Filters {
        fn type_info() -> SqliteTypeInfo {
            <[u8]>::type_info()
        }
    }
    #[sqlx(transparent)]
    struct FeedTtl(i64);
    #[automatically_derived]
    impl ::core::clone::Clone for FeedTtl {
        #[inline]
        fn clone(&self) -> FeedTtl {
            let _: ::core::clone::AssertParamIsClone<i64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FeedTtl {}
    #[automatically_derived]
    impl ::core::fmt::Debug for FeedTtl {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "FeedTtl", &&self.0)
        }
    }
    #[automatically_derived]
    impl<'q, DB: ::sqlx::Database> ::sqlx::encode::Encode<'q, DB> for FeedTtl
    where
        i64: ::sqlx::encode::Encode<'q, DB>,
    {
        fn encode_by_ref(
            &self,
            buf: &mut <DB as ::sqlx::database::HasArguments<'q>>::ArgumentBuffer,
        ) -> ::sqlx::encode::IsNull {
            <i64 as ::sqlx::encode::Encode<'q, DB>>::encode_by_ref(&self.0, buf)
        }
        fn produces(&self) -> Option<DB::TypeInfo> {
            <i64 as ::sqlx::encode::Encode<'q, DB>>::produces(&self.0)
        }
        fn size_hint(&self) -> usize {
            <i64 as ::sqlx::encode::Encode<'q, DB>>::size_hint(&self.0)
        }
    }
    #[automatically_derived]
    impl<'r, DB: ::sqlx::Database> ::sqlx::decode::Decode<'r, DB> for FeedTtl
    where
        i64: ::sqlx::decode::Decode<'r, DB>,
    {
        fn decode(
            value: <DB as ::sqlx::database::HasValueRef<'r>>::ValueRef,
        ) -> ::std::result::Result<
            Self,
            ::std::boxed::Box<
                dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
            >,
        > {
            <i64 as ::sqlx::decode::Decode<'r, DB>>::decode(value).map(Self)
        }
    }
    #[automatically_derived]
    impl<DB: ::sqlx::Database> ::sqlx::Type<DB> for FeedTtl
    where
        i64: ::sqlx::Type<DB>,
    {
        fn type_info() -> DB::TypeInfo {
            <i64 as ::sqlx::Type<DB>>::type_info()
        }
        fn compatible(ty: &DB::TypeInfo) -> ::std::primitive::bool {
            <i64 as ::sqlx::Type<DB>>::compatible(ty)
        }
    }
    impl Display for FeedTtl {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            humantime::format_duration(
                    std::time::Duration::from_secs(self.0.try_into().unwrap()),
                )
                .fmt(f)
        }
    }
    impl FromStr for FeedTtl {
        type Err = humantime::DurationError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(humantime::parse_duration(s)?.as_secs().try_into().unwrap()))
        }
    }
    struct UrlSource {
        pub source_link: String,
        pub ttl_seconds: FeedTtl,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UrlSource {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "UrlSource",
                "source_link",
                &self.source_link,
                "ttl_seconds",
                &&self.ttl_seconds,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UrlSource {
        #[inline]
        fn clone(&self) -> UrlSource {
            UrlSource {
                source_link: ::core::clone::Clone::clone(&self.source_link),
                ttl_seconds: ::core::clone::Clone::clone(&self.ttl_seconds),
            }
        }
    }
    struct Feed {
        pub id: i64,
        pub name: String,
        pub link_code: String,
        pub filters: Filters,
        #[sqlx(flatten)]
        pub url_source: Option<UrlSource>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Feed {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Feed",
                "id",
                &self.id,
                "name",
                &self.name,
                "link_code",
                &self.link_code,
                "filters",
                &self.filters,
                "url_source",
                &&self.url_source,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Feed {
        #[inline]
        fn clone(&self) -> Feed {
            Feed {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                link_code: ::core::clone::Clone::clone(&self.link_code),
                filters: ::core::clone::Clone::clone(&self.filters),
                url_source: ::core::clone::Clone::clone(&self.url_source),
            }
        }
    }
    #[automatically_derived]
    impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for Feed
    where
        &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
        i64: ::sqlx::decode::Decode<'a, R::Database>,
        i64: ::sqlx::types::Type<R::Database>,
        String: ::sqlx::decode::Decode<'a, R::Database>,
        String: ::sqlx::types::Type<R::Database>,
        String: ::sqlx::decode::Decode<'a, R::Database>,
        String: ::sqlx::types::Type<R::Database>,
        Filters: ::sqlx::decode::Decode<'a, R::Database>,
        Filters: ::sqlx::types::Type<R::Database>,
        Option<UrlSource>: ::sqlx::FromRow<'a, R>,
    {
        fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
            let id: i64 = row.try_get("id")?;
            let name: String = row.try_get("name")?;
            let link_code: String = row.try_get("link_code")?;
            let filters: Filters = row.try_get("filters")?;
            let url_source: Option<UrlSource> = <Option<
                UrlSource,
            > as ::sqlx::FromRow<'a, R>>::from_row(row)?;
            ::std::result::Result::Ok(Feed {
                id,
                name,
                link_code,
                filters,
                url_source,
            })
        }
    }
    impl Feed {}
}
use std::{future::ready, net::SocketAddr};
use anyhow::Context;
use axum::{routing::get, Router};
use sqlx::{migrate, SqlitePool};
use tower_http::services::ServeDir;
fn main() -> anyhow::Result<()> {
    let body = async {
        dotenvy::dotenv().context("Error loading .env file")?;
        env_logger::init();
        let database_url = std::env::var("DATABASE_URL")
            .context("Error loading DATABASE_URL")?;
        let pool = SqlitePool::connect(&database_url)
            .await
            .context("Error connecting to database")?;
        {
            ::sqlx::migrate::Migrator {
                migrations: ::std::borrow::Cow::Borrowed(
                    &[
                        ::sqlx::migrate::Migration {
                            version: 20230827144029i64,
                            description: ::std::borrow::Cow::Borrowed("feed"),
                            migration_type: ::sqlx::migrate::MigrationType::Simple,
                            sql: ::std::borrow::Cow::Borrowed(
                                "CREATE TABLE Feed (\n    id INTEGER NOT NULL PRIMARY KEY,\n    name TEXT NOT NULL,\n    link_code TEXT NOT NULL UNIQUE,\n    filters BLOB NOT NULL,\n    source_link TEXT,\n    ttl_seconds INTEGER,\n    last_update INTEGER,\n    CONSTRAINT link_code_lower CHECK (link_code = lower(link_code))\n);\n\nINSERT INTO Feed(name, link_code, filters, source_link, ttl_seconds) VALUES(\"Hello World\", \"446fc76e7cf24f8a819e600528860329\", \"[]\", \"https://example.com/feed.ical\", 21600);\nINSERT INTO Feed(name, link_code, filters) VALUES(\"Hatsune Miku\", \"d9d03e2fce5244768391cad3a9b6cb9a\", \"[]\");\n",
                            ),
                            checksum: ::std::borrow::Cow::Borrowed(
                                &[
                                    70u8,
                                    96u8,
                                    53u8,
                                    121u8,
                                    32u8,
                                    58u8,
                                    216u8,
                                    145u8,
                                    45u8,
                                    116u8,
                                    60u8,
                                    214u8,
                                    111u8,
                                    238u8,
                                    200u8,
                                    201u8,
                                    76u8,
                                    104u8,
                                    23u8,
                                    41u8,
                                    248u8,
                                    188u8,
                                    9u8,
                                    242u8,
                                    234u8,
                                    60u8,
                                    47u8,
                                    85u8,
                                    72u8,
                                    212u8,
                                    240u8,
                                    82u8,
                                    62u8,
                                    182u8,
                                    14u8,
                                    238u8,
                                    126u8,
                                    36u8,
                                    166u8,
                                    207u8,
                                    13u8,
                                    226u8,
                                    172u8,
                                    94u8,
                                    191u8,
                                    115u8,
                                    6u8,
                                    177u8,
                                ],
                            ),
                        },
                    ],
                ),
                ignore_missing: false,
                locking: true,
            }
        }
            .run(&pool)
            .await
            .context("Error running migration")?;
        let app = Router::new()
            .route("/", get(presentation::pages::root))
            .route("/feed/:id/status", get(presentation::pages::feed_status))
            .route("/feed/:id/edit", get(presentation::pages::feed_edit))
            .route("/export/:code", get(logic::export))
            .fallback_service(
                ServeDir::new("assets")
                    .not_found_service(get(|| ready(presentation::error::make_404()))),
            )
            .with_state(pool);
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("listening on http://{0}", addr),
                    lvl,
                    &("ical_manager", "ical_manager", "src/main.rs"),
                    42u32,
                    ::log::__private_api::Option::None,
                );
            }
        };
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .context("Error binding server")?;
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
