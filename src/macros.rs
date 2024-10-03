#[macro_export]
macro_rules! rich_text {
    ($text:expr) => {
        $crate::others::rich_text::RichText::from($text)
    };
}
