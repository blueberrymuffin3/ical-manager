pub trait IntoStrumStr {
    fn into_strum_str(self) -> &'static str;
}

impl<T> IntoStrumStr for T
where
    &'static str: From<T>,
{
    fn into_strum_str(self) -> &'static str {
        self.into()
    }
}
