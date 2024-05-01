use rmpv::Value;

#[derive(Debug, PartialEq)]
pub struct Item<'a> {
    pub id: i32,
    pub process_id: i32,
    pub thread_id: i32,
    pub timestamp_ns: i64,
    pub line: i32,
    pub value: f32,
    pub filename: &'a str,
    pub path: &'a str,
}

impl<'a> Item<'a> {
    pub fn from_list(arr: &'a [Value]) -> Item<'a> {
        Item {
            id: arr[0].as_i64().unwrap() as i32,
            process_id: arr[1].as_i64().unwrap() as i32,
            thread_id: arr[2].as_i64().unwrap() as i32,
            timestamp_ns: arr[3].as_i64().unwrap(),
            line: arr[4].as_i64().unwrap() as i32,
            value: arr[5].as_f64().unwrap() as f32,
            filename: arr[6].as_str().unwrap(),
            path: arr[7].as_str().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_list() {
        let arr: Vec<Value> = vec![
            Value::from(0),
            Value::from(1),
            Value::from(2),
            Value::from(3),
            Value::from(4),
            Value::from(5.0f32),
            Value::from("hello.rs"),
            Value::from("/tmp"),
        ];

        let item = Item::from_list(&arr);

        assert_eq!(item.id, 0);
        assert_eq!(item.process_id, 1);
        assert_eq!(item.thread_id, 2);
        assert_eq!(item.timestamp_ns, 3);
        assert_eq!(item.line, 4);
        assert_eq!(item.value, 5.0f32);
        assert_eq!(item.filename, "hello.rs");
        assert_eq!(item.path, "/tmp");
    }
}
