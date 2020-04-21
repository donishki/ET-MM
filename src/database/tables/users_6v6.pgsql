/*
 * 6v6 users table
 *
 * table containing information for registered 6v6 users.
 *
 * columns:
 *     user_id - unique database user id for relational purposes
 *     user_name - unique discord user name (per server)
 *     discord_uuid - discord unique user id snowflake
 *     account_active - whether or not the account is currently active
 */
CREATE TABLE IF NOT EXISTS users_6v6 (
    user_id SERIAL PRIMARY KEY,
    user_name TEXT UNIQUE NOT NULL,
    discord_uuid TEXT UNIQUE NOT NULL,
    account_active BOOLEAN NOT NULL
);
