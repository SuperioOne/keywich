pub struct Charset<'a>
{
    charset: &'a [u8],
}

impl<'a> Charset<'_>
{
    fn new() -> Charset<'a>
    {
        let test = &[0u8; 16];

        Charset {
            charset: test
        }
    }

    fn normalize(&self, content: &[u8]) -> String
    {
        self.charset;

        "".into()
    }
}
