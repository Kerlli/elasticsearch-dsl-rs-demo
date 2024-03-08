use serde::{
    Serialize,
    Serializer,
    ser::{
        SerializeMap,
        SerializeSeq,
    }
};
use serde_with::{SerializeDisplay, skip_serializing_none};
use macros::DisplayCase;

pub struct Sort {
    sorts: Vec<SortClause>,
}

impl Serialize for Sort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.sorts.len()))?;
        for s in self.sorts.iter() {
            seq.serialize_element(s)?;
        }
        seq.end()
    }
}

impl Sort {
    pub fn new(sort_clauses: Vec<SortClause>) -> Self {
        Self {
            sorts: sort_clauses,
        }
    }
}

#[derive(Clone)]
pub struct SortClause {
    field: String,
    opts: SortOptions,
}

impl SortClause {
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
            opts: Default::default(),
        }
    }

    pub fn format(&mut self, format: &str) -> &mut Self {
        self.opts.format = Some(format.to_string());

        self
    }

    pub fn order(&mut self, order: Order) -> &mut Self {
        self.opts.order = Some(order);

        self
    }

    pub fn mode(&mut self, mode: Mode) -> &mut Self {
        self.opts.mode = Some(mode);

        self
    }

    pub fn numberic_type(&mut self, numberic_type: NumbericType) -> &mut Self {
        self.opts.numberic_type = Some(numberic_type);

        self
    }
}

impl Serialize for SortClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.field, &self.opts)?;
        map.end()
    }
}

#[skip_serializing_none]
#[derive(Clone, Serialize)]
struct SortOptions {
    format: Option<String>,
    order: Option<Order>,
    mode: Option<Mode>,
    numberic_type: Option<NumbericType>,
    // nested,
}

impl Default for SortOptions {
    fn default() -> Self {
        Self {
            format: None,
            order: None,
            mode: None,
            numberic_type: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, DisplayCase, SerializeDisplay)]
#[display_case(case = "lowercase")]
pub enum Order {
    Asc,
    Desc,
}

#[allow(dead_code)]
#[derive(Clone, DisplayCase, SerializeDisplay)]
#[display_case(case = "lowercase")]
pub enum Mode {
    Min,
    Max,
    Sum,
    Avg,
    Median,
}

#[allow(dead_code)]
#[derive(Clone, DisplayCase, SerializeDisplay)]
#[display_case(case = "snakecase")]
pub enum NumbericType {
    Double,
    Long,
    Date,
    DateNanos,
}

#[macro_export]
macro_rules! sort {
    ($($clause:expr),*) => {
        {
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
/// Example
/// ```
/// use dsl::search::Search;
/// use dsl::sort::{Order, Mode, Sort, SortClause};
/// use dsl::{sort, sort_clause};
/// 
/// let search = Search::new()
///     .sort(
///         Sort::new(
///             sort!(
///                 sort_clause!("foo", order = Order::Asc, mode = Mode::Min),
///                 sort_clause!("bar")
///             )
///         )
///     );
/// ```
#[macro_export]
macro_rules! sort_clause {
    ($field:expr) => {
        SortClause::new($field)
    };

    ($field:expr $(, $key:ident = $value:expr)*) => {
        {
            SortClause::new($field)
                $(.$key($value))*
        }
    };
}
