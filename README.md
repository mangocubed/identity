# Mango³ ID (Identity)

Authentication provider and user account manager.

### Build Requirements

- Node.js 24.x
- Rust 1.93.x

## Run Requirements

- PostgreSQL 18.x
- Redis 8.x

## Environment variables

| Name                         | Type    | Default                                                          | Packages        |
| ---------------------------- | ------- | ---------------------------------------------------------------- | --------------- |
| API_ADDRESS                  | String  | 127.0.0.1:8005                                                   | api             |
| API_URL                      | String  | http://127.0.0.1:8005                                            | api             |
| APP_CLIENT_IP_SOURCE         | String  | ConnectInfo                                                      | app             |
| ACCESS_TOKEN_CODE_TTL_SECS   | Number  | 86400                                                            | api,app         |
| ACCESS_TOKEN_MIN_LENGTH      | Number  | 64                                                               | api,app         |
| ACCESS_TOKEN_MAX_LENGTH      | Number  | 128                                                              | api,app         |
| APPLICATION_TOKEN_TTL_SECS   | Number  | 2592000                                                          | api             |
| APPLICATION_TOKEN_MIN_LENGTH | Number  | 64                                                               | api             |
| APPLICATION_TOKEN_MAX_LENGTH | Number  | 128                                                              | api             |
| APPLICATION_TOKEN_TTL_SECS   | Number  | 31104000                                                         | api             |
| AUTHORIZATION_MIN_LENGTH     | Number  | 64                                                               | app             |
| AUTHORIZATION_MAX_LENGTH     | Number  | 128                                                              | app             |
| AUTHORIZATION_TTL_SECS       | Number  | 600                                                              | app,monitor     |
| DATABASE_MAX_CONNECTIONS     | Number  | 5                                                                | api,app,monitor |
| DATABASE_URL                 | String  | postgres://mango3:mango3@127.0.0.1:5432/identity_dev             | api,app,monitor |
| IP_GEO_API_KEY               | String  |                                                                  | monitor         |
| LEPTOS_SITE_ADDR             | String  | 127.0.0.1:8000                                                   | app             |
| MONITOR_REDIS_URL            | String  | redis://127.0.0.1:6379/1                                         | app,monitor     |
| SESSION_DOMAIN               | String  |                                                                  | app             |
| SESSION_PRIVATE_KEY          | String  | abcdefghijklmnopqrestuvvwxyz0123456789ABCDEFGHIJKLMNOPQRESTUVVWX | app             |
| SESSION_REDIS_URL            | String  | redis://127.0.0.1:6379/2                                         | app             |
| SESSION_SECURE               | Boolean | false                                                            | app             |
| STORAGE_FONT_PATH            | String  | /usr/share/fonts/truetype/dejavu/DejaVuSans.ttf                  | app             |
| STORAGE_PATH                 | String  | ./storage/                                                       | app             |

Other environment variables: https://github.com/mangocubed/toolbox#environment-variables
