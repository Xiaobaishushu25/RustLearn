#[cfg(test)]
mod test_cow{
    use std::borrow::Cow;
    use std::cell::OnceCell;

    const SENSITIVE_WORD:&str = "fuck";
    //写时复制，没有敏感词不会发生复制
    ///cow的定义，可以看出对于Cow<str>，B是str，所以Cow::Borrowed需要的是&str，Cow::Owned需要的是str
    ///pub enum Cow<'a, B: ?Sized + 'a>
    /// where
    ///     B: ToOwned,
    /// {
    ///     /// Borrowed data.
    ///     #[stable(feature = "rust1", since = "1.0.0")]
    ///     Borrowed(#[stable(feature = "rust1", since = "1.0.0")] &'a B),
    ///
    ///     /// Owned data.
    ///     #[stable(feature = "rust1", since = "1.0.0")]
    ///     Owned(#[stable(feature = "rust1", since = "1.0.0")] <B as ToOwned>::Owned),
    /// }
    fn replace_sensitive_word_cow(text:&str) -> Cow<str>{
        if text.contains(SENSITIVE_WORD){
            Cow::Owned(text.replace(SENSITIVE_WORD,"hello"))
        }else {
            Cow::Borrowed(text)
        }
    }
    fn replace_sensitive_word(text:&str) -> String{
        if text.contains(SENSITIVE_WORD){
            String::from(text.replace(SENSITIVE_WORD,"hello"))
        }else {
            String::from(text)
        }
    }
    #[test]
    fn test(){
        let text = "you bad,i fuck";
        let cow = replace_sensitive_word_cow(text);
        let string = replace_sensitive_word(text);
        println!("{cow}");
        println!("{string}");
    }
}