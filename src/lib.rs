use reqwest::Client;
use serde_json::json;
use transactions::{Transaction, Transactions};
use worker::*;

mod transactions;

#[event(fetch)]
async fn main(_: Request, _: Env, _: Context) -> Result<Response> {
    Response::ok("Hello, HCB!")
}

#[event(scheduled)]
pub async fn scheduled(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {}

async fn check_for_updates(env: Env) -> Result<String> {
    let kv = env.kv("HCB_TRANSACTIONS")?;

    let hcb_org_id = env.var("HCB_ORG_ID")?.to_string();
    let hcb_api_url = Url::parse(
        format!(
            "https://hcb.hackclub.com/api/v3/organizations/{}/transactions",
            &hcb_org_id
        )
        .as_str(),
    )?;

    let transactions = fetch_transactions(hcb_api_url).await?;
    let latest = transactions.first().unwrap();

    let last_stored_id = kv.get("latest").text().await.unwrap();

    match last_stored_id {
        Some(id) => {
            if id == latest.id {
                return Ok("No new transactions".to_string());
            } else {
                kv.put("latest", latest.clone().id)
                    .unwrap()
                    .execute()
                    .await
                    .unwrap();
                return Ok("New transactions found".to_string());

                // Send message to Slack Webhook
                let slack_webhook_url = env.secret("SLACK_WEBHOOK_URL")?.to_string();
                let message = format!(
                    "New transaction: {}",
                    serde_json::to_string(&latest).unwrap()
                );

                let request_body = &json!({ "text": message });

                let client = Client::new();
                client
                    .post(slack_webhook_url)
                    .body(request_body.to_string())
                    .send()
                    .await
                    .unwrap();

                return Ok(message);
            }
        }
        None => Err("No last stored ID found".into()),
    }
}

async fn fetch_transactions(url: Url) -> Result<Transactions> {
    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "HCB Transaction Monitor (github.com/ari-kl)")
        .send()
        .await
        .unwrap();

    let transactions: Transactions = serde_json::from_str(&response.text().await.unwrap())?;

    Ok(transactions)
}
