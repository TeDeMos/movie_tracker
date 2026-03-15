pub trait TryJoin {
    type Error;

    fn try_join(&mut self, sep: &str) -> Result<String, Self::Error>;
}

impl<E, I: Iterator<Item = Result<String, E>>> TryJoin for I {
    type Error = E;

    fn try_join(&mut self, sep: &str) -> Result<String, Self::Error> {
        let mut result = match self.next() {
            Some(s) => s?,
            None => return Ok(String::new()),
        };
        for i in self {
            result.push_str(sep);
            result.push_str(&i?);
        }
        Ok(result)
    }
}
