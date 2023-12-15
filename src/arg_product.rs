use crate::*;
use itertools::Itertools;
use std::collections::HashMap;
use toml::Value;

pub fn arg_product(args: &HashMap<String, Value>) -> Vec<QueryArgs> {
    let args_list: Vec<Vec<(String, Value)>> = args
        .into_iter()
        .map(|(k, v)| {
            let args_vec: Vec<Value> = match v {
                Value::Table(_) => panic!("Tables currently not supported"),
                Value::Array(args) => args.to_vec(),
                val => vec![val.clone()],
            };

            args_vec
                .into_iter()
                .map(|v| (k.to_owned(), v))
                .collect::<Vec<(String, Value)>>()
        })
        .collect();

    let args_vec: Vec<QueryArgs> = args_list
        .into_iter()
        .multi_cartesian_product()
        .map(|args| {
            let mut query_args = QueryArgs(HashMap::new());

            for (k, v) in args {
                query_args.0.insert(k, v);
            }

            query_args
        })
        .collect();

    args_vec
}

#[cfg(test)]
mod tests {
    use crate::arg_product::*;

    #[test]
    fn it_computes_args_for_two_lists() {
        let mut args = HashMap::new();

        args.insert(
            "foo".to_string(),
            Value::Array(vec!["1".into(), "2".into()]),
        );

        args.insert(
            "bar".to_string(),
            Value::Array(vec!["a".into(), "b".into()]),
        );

        let result = arg_product(&args);

        let expected = vec![
            QueryArgs(
                vec![
                    ("foo".to_string(), Value::String("1".to_string())),
                    ("bar".to_string(), Value::String("a".to_string())),
                ]
                .into_iter()
                .collect(),
            ),
            QueryArgs(
                vec![
                    ("foo".to_string(), Value::String("1".to_string())),
                    ("bar".to_string(), Value::String("b".to_string())),
                ]
                .into_iter()
                .collect(),
            ),
            QueryArgs(
                vec![
                    ("foo".to_string(), Value::String("2".to_string())),
                    ("bar".to_string(), Value::String("a".to_string())),
                ]
                .into_iter()
                .collect(),
            ),
            QueryArgs(
                vec![
                    ("foo".to_string(), Value::String("2".to_string())),
                    ("bar".to_string(), Value::String("b".to_string())),
                ]
                .into_iter()
                .collect(),
            ),
        ];

        assert!(expected.iter().all(|item| result.contains(item)));
    }
}
