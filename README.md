# HCB Transaction Tracker
This is a simple transaction tracker that uses the [HCB API](https://hcb.hackclub.com/docs/api/v3/) to check for new transactions, and sends them to a Slack webhook. It was originally created to track Arcade transactions (see #arcade-hcb-tracker on the Hack Club Slack)

The transactions tracker is hosted on Cloudflare Workers, and is scheduled to run every 5 minutes.

Before using this tracker, you need to set:
- The `HCB_ORG_ID` variable in the Cloudflare Workers environment to the ID of your HCB organization (`arcade` by default)
- The `SLACK_WEBHOOK_URL` secret in the Cloudflare Workers environment to the URL of your Slack webhook
- A KV namespace binded to `HCB_TRANSACTIONS`

You also need to set the value of a transaction ID to the `latest` key in the KV store. This doesn't have to be the actual latest ID, but is preferred to prevent spam.
