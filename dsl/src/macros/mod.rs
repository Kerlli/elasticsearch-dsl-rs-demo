#[macro_export]
macro_rules! match_clause {
    ($field:expr, $value:expr) => {
        {
            use $crate::query::LeafClause;

            LeafClause::Match(Match::new($field, $value))
        }
    };
}

#[macro_export]
macro_rules! range_clause {
    ($r:expr) => {
        {
            use $crate::query::LeafClause;

            LeafClause::Range($r)
        }
    };
}
