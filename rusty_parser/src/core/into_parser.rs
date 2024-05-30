pub trait IntoParser {
    type Into;

    fn into_parser(self) -> Self::Into;
}
