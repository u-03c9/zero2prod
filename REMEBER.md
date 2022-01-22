# Remember

- After creating the database, apply migrations on the remote database by running:
```sh
DATABASE_URL="{database connection string}" sqlx migrate run
```

- Change the environment key `APP_APPLCIATION__BASE_URL` to its real value.

- Create a new secret file in render called `email_client.yaml` to be merged with the app's config in runtime, and it should contains those values:

```yaml
email_client:
  base_url: {{ api url }}
  sender_email: {{ verfied email address }}
  authorization_token: {{ apikey }}
```