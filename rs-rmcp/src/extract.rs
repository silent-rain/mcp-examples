//! 提取器

use serde::de::DeserializeOwned;
use serde_json::{self, Value};
use std::collections::BTreeMap;

use crate::error::{Error, PathExtractionError};

/// 纯 URL 提取器
pub struct Path<T>(pub T)
where
    T: DeserializeOwned;

impl<T> Path<T>
where
    T: DeserializeOwned,
{
    /// 从 URL 和模式中提取参数，解析为目标类型 T
    #[allow(unused)]
    pub fn extract(url: &str, pattern: &str) -> Result<T, Error> {
        // 提取键值对
        let params = Self::extract_params(url, pattern)?;

        // 尝试解析为结构体（通过 serde）
        if let Ok(value) = serde_json::from_value::<T>(serde_json::to_value(params.clone())?) {
            return Ok(value);
        }

        // 尝试解析为元组类型
        if let Some(tuple_value) = Self::try_extract_tuple(&params)
            && let Ok(value) = serde_json::from_value::<T>(tuple_value)
        {
            return Ok(value);
        }

        Err(PathExtractionError::UnsupportedType.into())
    }

    /// 尝试将参数提取为元组值
    fn try_extract_tuple(params: &BTreeMap<String, Value>) -> Option<Value> {
        // 将参数值按顺序收集到数组中
        let values: Vec<Value> = params.values().cloned().collect();

        // 如果成功收集了所有值，则创建一个数组值（serde 会将其反序列化为元组）
        Some(Value::Array(values))
    }

    /// 从 URL 和模式中提取键值对
    fn extract_params(
        url: &str,
        pattern: &str,
    ) -> Result<BTreeMap<String, Value>, PathExtractionError> {
        let url_segments: Vec<&str> = url.split('/').filter(|s| !s.is_empty()).collect();
        let pattern_segments: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();

        if url_segments.len() != pattern_segments.len() {
            return Err(PathExtractionError::MissingParams);
        }

        let mut params = BTreeMap::new();

        for (url_seg, pattern_seg) in url_segments.iter().zip(pattern_segments.iter()) {
            if pattern_seg.starts_with('{') && pattern_seg.ends_with('}') {
                let key = &pattern_seg[1..pattern_seg.len() - 1];
                // 尝试将字符串转换为适当的类型
                let value = if let Ok(int_val) = url_seg.parse::<i64>() {
                    Value::Number(int_val.into())
                } else if url_seg.contains('.') {
                    // 确保这是一个浮点数（包含小数点）
                    if let Ok(float_val) = url_seg.parse::<f64>() {
                        Value::Number(serde_json::Number::from_f64(float_val).unwrap())
                    } else {
                        Value::String(url_seg.to_string())
                    }
                } else if *url_seg == "true" {
                    Value::Bool(true)
                } else if *url_seg == "false" {
                    Value::Bool(false)
                } else {
                    Value::String(url_seg.to_string())
                };
                params.insert(key.to_string(), value);
            } else if url_seg != pattern_seg {
                return Err(PathExtractionError::InvalidFormat);
            }
        }

        Ok(params)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn test_extract_hash_map() {
        let url = "/dynamic/resource/42/axum";
        let pattern = "/dynamic/resource/{id}/{name}";

        match Path::<BTreeMap<String, Value>>::extract(url, pattern) {
            Ok(v) => println!("{v:?}"),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_extract_struct() {
        let url = "/dynamic/resource/42/axum";
        let pattern = "/dynamic/resource/{id}/{name}";

        #[allow(dead_code)]
        #[derive(Debug, Deserialize)]
        struct Test {
            id: i32,
            name: String,
        }

        match Path::<Test>::extract(url, pattern) {
            Ok(v) => println!("{v:?}"),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_extract_tuple() {
        let url = "/dynamic/resource/42/axum";
        let pattern = "/dynamic/resource/{id}/{name}";

        // 测试提取为元组
        match Path::<(i32, String)>::extract(url, pattern) {
            Ok(v) => println!("Tuple: {:?}", v),
            Err(e) => println!("Error: {}", e),
        }

        // 测试提取为元组
        match Path::<(String, String)>::extract(url, pattern) {
            Ok(v) => println!("String Tuple: {:?}", v),
            Err(e) => println!("Error: {}", e),
        }
    }
}
