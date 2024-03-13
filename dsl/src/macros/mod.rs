/// Create a match clause like Python `**kwargs` function
/// 
/// # Example
/// ```
/// use dsl::{
///     match_clause,
///     query::{
///         bool::Bool,
///         Query,
///         QueryValue
///     },
///     search::Search,
/// };
/// 
/// let search = Search::new()
///     .query(
///         Query::new()
///             .bool(
///                 Bool::new()
///                     .must(
///                         match_clause!(
///                             "event.action",
///                             QueryValue::Text("logged-in".to_owned()),
///                             prefix_length = 3
///                         )
///                     )
///             )
///     )
///     .build();
/// ```
#[macro_export]
macro_rules! match_clause {
    ($field:expr, $value:expr) => {
        {
            use $crate::query::{
                LeafClause,
                r#match::Match,
            };

            LeafClause::Match(
                &Match::new($field.into(), $value)
            )
        }
    };

    ($field:expr, $value:expr $(, $k:ident = $v:expr)*) => {
        {
            use $crate::query::{
                LeafClause,
                r#match::Match,
            };
    
            LeafClause::Match(
                Match::new($field.into(), $value)
                $(.$k($v))*
            )
        }
    };
}

#[macro_export]
macro_rules! range_clause {
    ($field:expr) => {
        {
            use $crate::query::{
                LeafClause,
                range::Range,
            };

            LeafClause::Range(
                &Range::new($field.into())
            )
        }
    };

    ($field:expr $(, $key:ident = $value:expr)*) => {
        {
            use $crate::query::{
                LeafClause,
                range::Range,
            };

            LeafClause::Range(
                Range::new($field.into())
                $(.$key($value))*
            )
        }
    };
}

#[macro_export]
macro_rules! term_clause {
    ($field:expr, $value:expr) => {
        {
            use $crate::query::{
                LeafClause,
                term::Term,
            };

            LeafClause::Term(
                &Term::new($field.into(), $value)
            )
        }
    };

    ($field:expr, $value:expr $(, $k:ident = $v:expr)*) => {
        {
            use $crate::query::{
                LeafClause,
                term::Term,
            };

            LeafClause::Range(
                Range::new($field.into(), $value)
                $(.$k($v))*
            )
        }
    };
}

#[macro_export]
macro_rules! sort {
    ($($clause:expr),*) => {
        {
            use $crate::sort::SortClause;

            let mut clauses: Vec<SortClause> = vec![];

            $(
                clauses.push($clause.clone());
            )*

            clauses
        }
    };
}

/// Create a sort clause like Python `**kwargs` function
/// 
/// # Example
/// ```
/// use dsl::search::Search;
/// use dsl::sort::{Order, Mode, SortClause};
/// use dsl::{sort, sort_clause};
/// 
/// let search = Search::new()
///     .sort(
///         sort!(
///             sort_clause!("foo", order = Order::Asc, mode = Mode::Min),
///             sort_clause!("bar")
///         )
///     );
/// ```
#[macro_export]
macro_rules! sort_clause {
    ($field:expr) => {
        {
            use $crate::sort::SortClause;

            SortClause::new($field)
        }
    };

    ($field:expr $(, $key:ident = $value:expr)*) => {
        {
            use $crate::sort::SortClause;

            SortClause::new($field)
                $(.$key($value))*
        }
    };
}
