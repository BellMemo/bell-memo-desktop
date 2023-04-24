use serde::Serialize;

/// 通用响应数据结构
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response<T: Serialize> {
  pub code: i32,
  pub message: String,
  pub data: Option<T>,
}

impl<T> Response<T>
where
  T: Serialize,
{
  ///  创建一个返回响应数据结构
  pub fn new(code: i32, message: String, data: Option<T>) -> Self {
    Self {
      code,
      message,
      data,
    }
  }

  /// 内置的成功数据结构
  pub fn success(data: T) -> Self {
    Self {
      code: 0,
      message: "Success".to_string(),
      data: Some(data),
    }
  }

  /// 内置的失败数据结构
  pub fn fail(code: i32, message: String) -> Self {
    Self {
      code,
      message,
      data: None,
    }
  }
}