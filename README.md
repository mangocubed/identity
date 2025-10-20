# Mango³ ID (Identity)

Authentication provider and user account manager.

## Environment variables

| Name                           | Type    | Default                                              | Client-side |
| ------------------------------ | ------- | ---------------------------------------------------- | ----------- |
| API_ADDRESS                    | String  | 127.0.0.1:8082                                       | No          |
| APP_REQUEST_ADDRESS            | String  | 127.0.0.1:8081                                       | No          |
| APP_REQUEST_URL                | String  | http://127.0.0.1:8081                                | Yes         |
| APP_OLD_TOKENS                 | Array   | []                                                   | No          |
| APP_TOKEN                      | String  |                                                      | Yes         |
| APP_TITLE                      | String  | Mango³                                               | Yes         |
| APPLICATIONS_TOKEN_LENGTH      | Integer | 32                                                   | No          |
| APPLICATIONS_SECRET_LENGTH     | Integer | 32                                                   | No          |
| DATABASE_MAX_CONNECTIONS       | Integer | 5                                                    | No          |
| DATABASE_URL                   | String  | postgres://mango3:mango3@127.0.0.1:5432/identity_dev | No          |
| IP_GEOLOCATION_API_KEY         | String  |                                                      | No          |
| MAILER_ENABLE                  | Boolean | false                                                | No          |
| MAILER_SENDER_ADDRESS          | String  | Mango³ dev <no-reply@localhost>                      | No          |
| MAILER_SMTP_ADDRESS            | String  | localhost                                            | No          |
| MAILER_SMTP_PASSWORD           | String  |                                                      | No          |
| MAILER_SMTP_SECURITY           | String  | none                                                 | No          |
| MAILER_SMTP_USERNAME           | String  |                                                      | No          |
| MAILER_SUPPORT_EMAIL_ADDRESS   | String  | support@localhost                                    | No          |
| MONITOR_REDIS_URL              | String  | redis://localhost:6379/0                             | No          |
| STORAGE_FILE_KEY_DURATION_SECS | Integer | 60                                                   | No          |
| STORAGE_IMAGE_FILTER_TYPE      | String  | CatmullRom                                           | No          |
| STORAGE_MAX_SIZE_MIB_PER_FILE  | Integer | 100                                                  | No          |
| STORAGE_PATH                   | String  | ./storage                                            | No          |
| USERS_SESSION_TOKEN_LENGTH     | Integer | 64                                                   | No          |
| USERS_LIMIT                    | Integer | 10                                                   | No          |
