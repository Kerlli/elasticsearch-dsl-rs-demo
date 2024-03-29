### A toy project for Elasticsearch DSL investigation

---

**Note**: This project is designed to work with Elasticsearch `v7.14` and is intended solely for personal exploration. It does not guarantee compatibility or suitability for production environments.

---

### Usage

```rust
use elasticsearch::cat::CatAliasesParts;
use dsl::{
    clause,
    query::{
        bool::Bool,
        range::RangeValue,
        Query,
        QueryValue
    },
    search::Search,
    sort::{
        nested::SortNested,
        Order,
        Sort
    },
    sort_clause,
    types::number::Number,
};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search = Search::new()
        .query(
            Query::new()
                .bool(
                    Bool::new()
                        .must(
                            clause!(
                                Match,
                                "event.action",
                                QueryValue::Text("logged-in".to_owned())
                            )
                        )
                        .filter(
                            clause!(
                                Range,
                                "@timestamp",
                                gte = RangeValue::Date("now-1h/H".to_owned()),
                                lte = RangeValue::Date("now/H".to_owned()),
                                format = "HH:mm:ss yyyy/MM/DD"
                            )
                        )
                )
        )
        .sort(
            Sort::new()
                .sort(
                    sort_clause!(
                        "@timestamp",
                        order = Order::Desc,
                        nested = SortNested::new("parent")
                            .filter(
                                clause!(
                                    Range,
                                    "parent.age",
                                    gte = RangeValue::Number(Number::I32(21))
                                )
                            )
                            .clone()
                    )
                )
        )
        .build();

    // search body
    dbg!("{}", &search);

    let r = client
        .search(elasticsearch::SearchParts::Index(&["my_index"]))
        .from(0)
        .size(5)
        .body(search)
        .send()
        .await?;

    let body = r.json::<Value>().await?;
    let took = body["took"].as_i64().unwrap();

    for hit in body["hits"]["hits"].as_array().unwrap() {
        println!("{:?}", hit["_source"]);
    }

    Ok(())
}
```

---

### LICENSE

[MIT](LICENSE)
