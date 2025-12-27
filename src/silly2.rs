pub struct List<'a, T> {
    pub data: T,
    pub nrev: Option<&'a List<'a, T>>,
}
