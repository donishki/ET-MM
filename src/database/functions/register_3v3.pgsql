/*
 * create the register_3v3 function used to register
 * new 3v3 users to the database.
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
CREATE OR REPLACE FUNCTION register_3v3 (
   	user_name TEXT,
    discord_uuid TEXT,
)
RETURNS INTEGER AS $$
DECLARE
    user_id BIGINT;
BEGIN
    IF EXISTS (
        SELECT 1
          FROM users_3v3 u3
         WHERE u3.user_name = LOWER($1)
    )
    THEN
        RETURN 1;
    END IF;
    IF EXISTS (
        SELECT 1
          FROM users_3v3 u3
         WHERE u3.discord_uuid = LOWER($2)
    )
    THEN
        RETURN 2;
    END IF;
    INSERT INTO users_3v3 (user_name, discord_uuid, account_active)
        VALUES (LOWER($1), LOWER($2), TRUE);
    RETURN 0;
END;
$$ LANGUAGE plpgsql;
