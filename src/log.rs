use serde::Serialize;
use serde_json::Value;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#{derive(Serialize)}
// Make it as flat as possible
struct RequestLogLine {
    uuid: String,
    timestamp: String, // ios8601
    // User and context attributes
    user_id: Option<u64>,
    // Http request attributes
    req_path: String,
    req_method: String,

    // Error attributes
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
