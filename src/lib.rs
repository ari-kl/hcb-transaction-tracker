use reqwest::Client;
use serde_json::json;
use transactions::{CardCharge, Transactions};
use worker::*;

mod transactions;

const USER_AGENT: &str = "HCB Transaction Monitor (github.com/ari-kl)";

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _: Context) -> Result<Response> {
    let router = Router::new();

    router
        .on_async("/", |_, ctx| async move {
            let body = check_for_updates(ctx.env).await?;
            Response::ok(body)
        })
        .run(req, env)
        .await
}

#[event(scheduled)]
pub async fn scheduled(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    check_for_updates(env).await.unwrap();
}

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
    let slack_webhook_url = env.secret("SLACK_WEBHOOK_URL")?.to_string();
    let transactions = fetch_transactions(hcb_api_url).await?;
    let client = Client::new();

    let mut new_transaction_ids: Vec<String> = Vec::new();

    for transaction in transactions.iter() {
        // Skip transaction if already seen
        let Ok(new_transaction) = kv.get(&transaction.id).text().await else {
            continue;
        };

        match new_transaction {
            Some(_) => continue,
            None => kv
                .put(&transaction.id, "true")
                .unwrap()
                .execute()
                .await
                .unwrap(),
        }

        let transaction_link_url = format!(
            "https://hcb.hackclub.com/hcb/{}",
            transaction.clone().id.split_off(4) // Cut off txn_ prefix
        );

        let mut message = format!(
            "*<{}|NEW TRANSACTION>*
*Date*: {}
*Memo*: {}
*Balance Change*: ${}",
            transaction_link_url,
            transaction.date,
            transaction.memo,
            transaction.amount_cents as f64 / 100.0
        );

        if let Some(card_charge) = &transaction.card_charge {
            let card_charge_url = Url::parse(&card_charge.href)?;
            let response = client
                .get(card_charge_url)
                .header("User-Agent", USER_AGENT)
                .send()
                .await
                .unwrap();

            let card_charge: CardCharge = serde_json::from_str(&response.text().await.unwrap())?;
            let user = &card_charge.user;

            message += &format!("\n*User*: {}", user.full_name);
        }

        let request_body = &json!({ "text": message });

        client
            .post(&slack_webhook_url)
            .body(request_body.to_string())
            .send()
            .await
            .unwrap();

        new_transaction_ids.push(transaction.id.clone());
    }

    Ok("New transactions: ".to_string() + &new_transaction_ids.join(", "))
}

async fn fetch_transactions(url: Url) -> Result<Transactions> {
    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .unwrap();

    let transactions: Transactions = serde_json::from_str(&response.text().await.unwrap())?;

    Ok(transactions)
}
