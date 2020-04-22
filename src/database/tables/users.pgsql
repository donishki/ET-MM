/*
 * users table
 *
 * table containing information for registered users. This table
 * is automatically populated when users join any of the active
 * match making groups. 
 *
 * columns:
 *     user_id: unique database user id for relational purposes
 *     discord_uuid: discord unique user id snowflake
 */
CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY,
    discord_uuid TEXT PRIMARY KEY
);
