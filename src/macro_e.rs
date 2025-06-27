#![allow(dead_code)]

use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn example() {
    println!("hello world");

    let test = call_tool("test_tool", serde_json::json!({"a": "hello", "b": 42}));
    let better_test = call_tool("better_test_tool", serde_json::json!({ "a": "hello", "b": 42, "c": 1337, "d": 101 }));

    println!("test tool result: {:?}", test);
    println!("better test tool result: {:?}", better_test);
}

fn test_tool(a: String, b: u32) -> Result<u32> {
    println!("a: {}, b: {}", a, b);
    Ok(42)
}
fn better_test_tool(a: String, b: u32, c: u32, d: u32) -> Result<u32> {
    println!("a: {}, b: {} and the rest: {} {}", a, b, c, d);
    Ok(42)
}

fn call_tool(tool_name: &str, args: serde_json::Value) -> Result<serde_json::Value> {
    match tool_name {
        "test_tool" => Ok(serde_json::to_value(test_tool.call((
            get_value(&args, "a")?,
            get_value(&args, "b")?,
        ))?)?),
        "better_test_tool" => Ok(serde_json::to_value(better_test_tool.call((
            get_value(&args, "a")?,
            get_value(&args, "b")?,
            get_value(&args, "c")?,
            get_value(&args, "d")?,
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

define_tuple_calls!(A, B, C, D);

macro_rules! define_tuple_calls {
    () => {};
    ($A:ident $(, $T:ident)* $(,)?) => {
        paste::paste! {
            trait [<Call $A $($T)*>]<$A, $($T,)* Z> {
                fn call(self, args: ($A, $($T,)*)) -> Z;
            }
            
            impl<F, $A, $($T,)* Z> [<Call $A $($T)*>]<$A, $($T,)* Z> for F
            where
                F: FnOnce($A, $($T,)*) -> Z,
            {
                #[allow(non_snake_case)]
                fn call(self, ($A, $($T,)*): ($A, $($T,)*)) -> Z {
                    self($A, $($T,)*)
                }
            }
        }
        define_tuple_calls!($($T,)*);
    };
}
use define_tuple_calls;
