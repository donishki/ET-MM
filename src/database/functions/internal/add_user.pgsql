/*
 * add user to database
 *
 * args:
 *    discord_uuid - user discord unique user id
 *
 * returns:
 *     success: 0
 *     failure (discord uuid already exists): 1
 */
CREATE OR REPLACE FUNCTION add_user (
    discord_uuid TEXT
)
RETURNS INTEGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1
          FROM users u
         WHERE u.discord_uuid = LOWER($1)
    )
    THEN
        RETURN 1;
    END IF;
    INSERT INTO users (discord_uuid)
        VALUES (LOWER($1));
    RETURN 0;
END;
$$ LANGUAGE plpgsql;
