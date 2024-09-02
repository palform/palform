use serde::Serialize;

pub trait WebhookManager<R: Serialize> {
    fn target_url(&self) -> url::Url;
    fn target_method(&self) -> reqwest::Method;
    fn request_body(&self) -> Option<R>;
}

pub async fn send_webhook<R, M>(manager: M) -> Result<(), reqwest::Error>
where
    R: Serialize,
    M: WebhookManager<R>,
{
    let c = reqwest::Client::new();
    let mut req = c.request(manager.target_method(), manager.target_url());

    let body = manager.request_body();
    if let Some(body) = body {
        req = req.json(&body);
    }

    req = req.header("User-Agent", "Palform");
    req.send().await.map(|_| ())
}
