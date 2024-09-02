#[macro_export]
macro_rules! into_outcome {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => return rocket::outcome::Outcome::Error(::std::convert::From::from(e)),
        }
    };
    ($expr:expr, $req:ident) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                $req.local_cache(|| e.clone());
                return rocket::outcome::Outcome::Error(::std::convert::From::from(e));
            }
        }
    };
}
