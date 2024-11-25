# Ntex with Turso

Build web server with the #1 Fastest Framework, and Scale your Database with Turso.

## Requirements

- [rust](https://www.rust-lang.org/tools/install)
- [libsql](https://crates.io/crates/libsql)

## Cli tools

- [turso cli](https://docs.turso.tech/cli/installation)
- [geni](https://crates.io/crates/geni)



<details>

<summary> 1. Set Up Turso Credentials</summary>

1. Signup at turso (optional if you already have an account)

```sh
turso auth signup
```


2. Login to your turso cli

```sh
turso auth login
```

3. create database 

```sh
turso db create <database-name>
```
4. get your `DATABASE_URL`

```sh
turso db show --url <database-name>
```

5. get your `DATABASE_TOKEN`

```sh
turso db tokens create <database-name>
```

6. update  `.env`

```sh
DATABASE_URL=libsql://[dbname]-[username].turso.io
DATABASE_TOKEN=your_token
```

</details>


<details> 
<summary> 2. Managing Migrations with `geni`</summary>

1. export ENV 

```sh
export DATABASE_URL=libsql://[dbname]-[username].turso.io
export DATABASE_TOKEN=[token]
```

2. Create new Migration

```sh
geni new create_users_table
```

3. Fill up your *.up.sql and *.down.sql schema

*.up.sql

```sql
CREATE TABLE users (
ID INTEGER PRIMARY KEY AUTOINCREMENT,
name TEXT
);
```

*.down.sql

```sql
DROP TABLE users;
```

4. Run your migration (optional)

when we run our app it would automatically run the migration.

```sh
geni up
```

</details>

## References

[turso rust sdk](https://docs.turso.tech/sdk/rust/reference)

[local development](https://docs.turso.tech/local-development)


[encryption at production](https://docs.turso.tech/libsql#encryption-at-rest)

[ntex](https://ntex.rs/docs/getting-started)

