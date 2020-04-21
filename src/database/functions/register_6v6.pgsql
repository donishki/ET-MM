/*
 * create the register_6v6 function used to register
 * new 6v6 users to the database.
 *
 * args:
 *    user_name - registree username
 *    discord_uuid - registree discord unique user id
 *
 * returns:
 *     success: 0
 *     failure (username already exists): 1
 *     failure (discord uuid already exists): 2
 */
CREATE OR REPLACE FUNCTION register_6v6 (
   	user_name TEXT,
    discord_uuid TEXT,
)
RETURNS INTEGER AS $$
DECLARE
    user_id BIGINT;
BEGIN
    IF EXISTS (
        SELECT 1
          FROM users_6v6 u6
         WHERE u6.user_name = LOWER($1)
    )
    THEN
        RETURN 1;
    END IF;
    IF EXISTS (
        SELECT 1
          FROM users_6v6 u6
         WHERE u6.discord_uuid = LOWER($2)
    )
    THEN
        RETURN 2;
    END IF;
    INSERT INTO users_6v6 (user_name, discord_uuid, account_active)
        VALUES (LOWER($1), LOWER($2), TRUE);
    RETURN 0;
END;
$$ LANGUAGE plpgsql;
