# Mango³ ID (Identity)

Authentication provider and user account manager.

## Environment variables

| Name                           | Type    | Default                                              |
| ------------------------------ | ------- | ---------------------------------------------------- |
| APP_TOKEN                      | String  | identity_dev                                         |
| DATABASE_MAX_CONNECTIONS       | Integer | 5                                                    |
| DATABASE_URL                   | String  | postgres://mango3:mango3@127.0.0.1:5432/identity_dev |
| IP_GEOLOCATION_API_KEY         | String  |                                                      |
| MAILER_ENABLE                  | Boolean | false                                                |
| MAILER_SENDER_ADDRESS          | String  | Mango³ dev <no-reply@localhost>                      |
| MAILER_SMTP_ADDRESS            | String  | localhost                                            |
| MAILER_SMTP_PASSWORD           | String  |                                                      |
| MAILER_SMTP_SECURITY           | String  | none                                                 |
| MAILER_SMTP_USERNAME           | String  |                                                      |
| MAILER_SUPPORT_EMAIL_ADDRESS   | String  | support@localhost                                    |
| MONITOR_REDIS_URL              | String  | redis://localhost:6379/0                             |
| STORAGE_FILE_KEY_DURATION_SECS | Integer | 60                                                   |
| STORAGE_IMAGE_FILTER_TYPE      | String  | CatmullRom                                           |
| STORAGE_MAX_SIZE_MIB_PER_FILE  | Integer | 100                                                  |
| STORAGE_PATH                   | String  | ./storage                                            |
| USERS_SESSION_TOKEN_LENGTH     | Integer | 64                                                   |
| USERS_LIMIT                    | Integer | 10                                                   |
