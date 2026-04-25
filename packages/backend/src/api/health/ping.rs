use apistos::api_operation;

#[api_operation(tag = "Health", operation_id = "health.ping")]
pub async fn health_ping_handler() -> String {
    "all is healthy".to_string()
}
