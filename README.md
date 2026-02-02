# weather-lambda

A small Rust Lambda that sends the daily weather forecast to Discord every morning.

## What it does

Fetches weather data from [wttr.in](https://wttr.in) and posts a formatted summary to a Discord channel via webhook. Runs on a schedule using AWS EventBridge.

## Setup

1. Create a Discord webhook in your server (Server Settings → Integrations → Webhooks)

2. Copy `.env.example` to `.env` and add your webhook URL:
   ```
   DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/...
   ```

3. Deploy:
   ```bash
   make deploy
   ```

## Local testing

```bash
cargo run --features local
```

## Configuration

Edit `terraform/terraform.tfvars`:

| Variable | Description | Default |
|----------|-------------|---------|
| `discord_webhook_url` | Your Discord webhook | - |
| `schedule_expression` | When to run | `cron(0 7 * * ? *)` (7am daily) |
| `schedule_timezone` | Timezone | `Europe/Paris` |

## Requirements

- Rust
- [cargo-lambda](https://www.cargo-lambda.info/)
- Terraform
- AWS credentials configured
