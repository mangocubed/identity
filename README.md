# Mango³ ID (Identity)

Authentication provider and user account manager.

## Environment variables

| Name                           | Type    | Default                                              |
| ------------------------------ | ------- | ---------------------------------------------------- |
| APP_TOKEN                      | String  | 00000000                                             |
| DATABASE_MAX_CONNECTIONS       | Integer | 5                                                    |
| DATABASE_URL                   | String  | postgres://mango3:mango3@127.0.0.1:5432/identity_dev |
| STORAGE_FILE_KEY_DURATION_SECS | Integer | 60                                                   |
| STORAGE_IMAGE_FILTER_TYPE      | String  | CatmullRom                                           |
| STORAGE_MAX_SIZE_MIB_PER_FILE  | Integer | 100                                                  |
| STORAGE_PATH                   | String  | ./storage                                            |
| USERS_ACCESS_TOKEN_LENGTH      | Integer | 32                                                   |
| USERS_LIMIT                    | Integer | 10                                                   |
