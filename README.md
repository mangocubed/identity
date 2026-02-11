# Mango³ ID (Identity)

Authentication provider and user account manager.

### Build Requirements

- Node.js 24.x
- Rust 1.93.x

## Environment variables

| Name                         | Type    | Default                                              | Packages    |
| ---------------------------- | ------- | ---------------------------------------------------- | ----------- |
| CACHE_REDIS_URL              | String  | redis://127.0.0.1:6379/1                             | app,monitor |
| CACHE_TTL                    | Number  | 3600                                                 | app,monitor |
| DATABASE_MAX_CONNECTIONS     | Number  | 5                                                    | app,monitor |
| DATABASE_URL                 | String  | postgres://mango3:mango3@127.0.0.1:5432/identity_dev | app,monitor |
| IP_GEO_API_KEY               | String  |                                                      | monitor     |
| LEPTOS_SITE_ADDR             | String  | 127.0.0.1:8000                                       | app         |
| MAILER_ENABLE                | Boolean | false                                                | monitor     |
| MAILER_SENDER_ADDRESS        | String  | Mango³ dev <no-reply@localhost>                      | monitor     |
| MAILER_SMTP_ADDRESS          | String  | localhost                                            | monitor     |
| MAILER_SMTP_PASSWORD         | String  |                                                      | monitor     |
| MAILER_SMTP_SECURITY         | String  | none                                                 | monitor     |
| MAILER_SMTP_USERNAME         | String  |                                                      | monitor     |
| MAILER_SUPPORT_EMAIL_ADDRESS | String  | support@localhost                                    | monitor     |
| MONITOR_REDIS_URL            | String  | redis://127.0.0.1:6379/0                             | app,monitor |
