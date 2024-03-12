use super::LeafClause;
use serde::Serialize;
use serde_with::skip_serializing_none;

macro_rules! declare_bool {
    ($($field:ident),*) => {
        #[skip_serializing_none]
        #[derive(Serialize)]
        pub struct Bool<'a> {
            $($field: Option<Vec<LeafClause<'a>>>,)*
        }

        impl<'a> Bool<'a> {
            pub fn new() -> Self {
                Self {
                    $($field: None,)*
                }
            }

            $(
                #[allow(dead_code)]
                pub fn $field(&mut self, clause: LeafClause<'a>) -> &mut Self {
                    if self.$field.is_none() {
                        self.$field = Some(vec![clause]);
                    } else {
                        self.$field.as_mut().unwrap().push(clause);
                    }
            
                    self
                }
            )*
        }
    };
}

declare_bool!(must, filter, should, must_not);




