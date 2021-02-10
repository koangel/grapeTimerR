pub mod errors {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug, Clone)]
    pub enum TErrorKind {
        BadFormat,
        DateOverflow,
        WeekDay,
        AllocTickerError,
        Other(String),
    }

    #[derive(Debug,Clone)]
    pub struct TError {
        kind:TErrorKind,
        error:String,
    }

    impl TError {
        pub fn new(vkind:TErrorKind) -> TError {
            TError { kind:vkind.clone(),
                error: match vkind {
                TErrorKind::BadFormat => { String::from("error,bad date format...") }
                TErrorKind::DateOverflow => {  String::from("error,Date overflow...") }
                TErrorKind::WeekDay => { String::from("error,bad week day...") }
                TErrorKind::AllocTickerError  => { String::from("bad alloc ticker...") }
                TErrorKind::Other(v) => { v.clone() }
            } }
        }

        pub fn kind(&self) -> &TErrorKind {
            &self.kind
        }

        pub fn last_error(&self) -> &String {
            &self.error
        }
    }

    impl Error for TError {}

    impl fmt::Display for TError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"{}",self.error)
        }
    }
}