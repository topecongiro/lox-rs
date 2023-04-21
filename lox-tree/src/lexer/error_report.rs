pub trait ErrorReporter {
    fn report_error(&self, error: String);
}