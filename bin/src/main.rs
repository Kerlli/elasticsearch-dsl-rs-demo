use std::collections::HashMap;
use std::env;
use elasticsearch::{
    cat::CatAliasesParts,
    http::transport::Transport,
    Elasticsearch,
};
use serde::Deserialize;
use serde_json::Value;
use dsl::{
    clause,
    query::{
        bool::Bool,
        range::RangeValue,
        Query,
        QueryValue
    },
    search::Search,
    sort::Order,
    sort_clause,
    sort,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let es_url = env::var("ES_URL")
        .expect("Failed to read env variable `ES_URL`");

    let transport = Transport::single_node(&es_url)?;
    let client = Elasticsearch::new(transport);

    let aliases = get_all_aliases(&client)
        .await
        .unwrap_or(Vec::new());

    let mut alias_map: HashMap<String, Vec<String>> = HashMap::new();

    for alias in aliases {
        if let Some(indices) = alias_map.get_mut(&alias.alias) {
            indices.push(alias.index);
        } else {
            alias_map.insert(alias.alias, vec![alias.index]);
        }
    }

    let alias_list: Vec<&String> = alias_map
        .keys()
        .collect();

    let first_index = alias_list.first().unwrap();

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
            sort!(
                sort_clause!("@timestamp", order = Order::Desc)
            )
        )
        .build();

    dbg!("{}", &search);

    let r = client
        .search(elasticsearch::SearchParts::Index(&[first_index]))
        .from(0)
        .size(5)
        .body(search)
        .send()
        .await?;

    let body = r.json::<Value>().await?;
    for hit in body["hits"]["hits"].as_array().unwrap() {
        println!("{:?}", hit["_source"]);
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct CatAliasRecord {
    alias: String,
    index: String,
}

async fn get_all_aliases(client: &Elasticsearch) -> Option<Vec<CatAliasRecord>> {
    let response = client
        .cat()
        .aliases(CatAliasesParts::Name(&["*", "-.*"]))
        .format("json")
        .send()
        .await;

    if let Ok(response) = response {
        let response_body = response.json::<Vec<CatAliasRecord>>().await;
        if let Ok(aliases) = response_body {
            return Some(aliases)
        }
    }

    None
}
