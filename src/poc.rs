use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn example() {
    println!("hello world");

    let res = call_tool("test_tool", serde_json::json!({"a": "hello", "b": 42}));
    println!("result: {:?}", res);
}

fn test_tool(a: String, b: u32) -> Result<u32> {
    println!("a: {}, b: {}", a, b);
    Ok(42)
}

fn call_tool(tool_name: &str, args: serde_json::Value) -> Result<serde_json::Value> {
    match tool_name {
        "test_tool" => Ok(serde_json::to_value(test_tool.call((
            get_value(&args, "a")?,
            get_value(&args, "b")?,
        ))?)?),
        _ => Err("no valid tool name".into())
    }
}

fn get_value<T: for <'a> Deserialize<'a>>(root: &serde_json::Value, name: &str) -> Result<T> {
    Ok(serde_json::from_value(
        root
            .get(name)
            .ok_or(format!("Missing or invalid '{}' parameter", name))?
            .clone()
    )?)
}

trait Call2<A, B, Z> {
    fn call(self, args: (A, B)) -> Z;
}

impl<F, A, B, Z> Call2<A, B, Z> for F
where
    F: FnOnce(A, B) -> Z,
{
    fn call(self, (a, b): (A, B)) -> Z {
        self(a, b)
    }
}
