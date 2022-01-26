# Remember

- After creating the database, apply migrations on the remote database by running:
```sh
DATABASE_URL="{database connection string}" sqlx migrate run
```

- After creating new migrations to the database, make sure that `sqlx-data.json` exits and up-to-date by running:
```sh
cargo sqlx prepare -- --lib
```

- Change the environment key `APP_APPLICATION__BASE_URL` to its real value.

- Create a new secret file in render called `email_client.yaml` to be merged with the app's config in runtime, and it should contains those values:

```yaml
email_client:
  base_url: {{ api url }}
  sender_email: {{ verified email address }}
  sender_name: {{ will appear in the received emails }}
  authorization_token: {{ apikey }}
application:
  hmac_secret: {{ super long random key to verify message integrity }}
```