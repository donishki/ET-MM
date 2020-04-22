/*
 * add (or activate) a user to a matchmaking group
 *
 * args:
 *    discord_uuid: user discord unique user id
 *    group_name: match making group name
 *
 * returns:
 *     success: 0
 *     failure (failed to add user to database): 1
 *     failure (user is already added to group): 2
 */
CREATE OR REPLACE FUNCTION add_match_making_user (
    discord_uuid TEXT,
    group_name TEXT
)
RETURNS INTEGER AS $$
DECLARE
    i BIGINT,
    user_id BIGINT,
    group_id BIGINT;
BEGIN
    -- if user is not in users table add them
    IF NOT EXISTS (
        SELECT 1
          FROM users u
         WHERE u.discord_uuid = LOWER($1)
    )
    THEN
        SELECT add_user($1)
          INTO i;
        IF i != 0 THEN
            RETURN 1;
        END IF;
    END IF;
    -- check if user and group combination already exists
    IF EXISTS (
        SELECT 1
          FROM match_making_users mmu
         INNER JOIN users u ON mmu.user_id = u.user_id
         INNER JOIN match_making_groups mmg ON mmu.group_id = mmg.group_id
         WHERE u.discord_uuid = LOWER($1)
           AND mmg.group_name = LOWER($2)
    )
    THEN
        RETURN 2;
    END IF;
    -- insert values into table
    SELECT u.user_id
      FROM users u
     WHERE u.discord_uuid = LOWER($1)
      INTO user_id;
    SELECT mmg.group_id
      FROM match_making_groups mmg
     WHERE mmg.group_name = LOWER($2)
      INTO group_id;
    INSERT INTO match_making_users (user_id, group_id)
        VALUES (user_id, group_id);
    RETURN 0;
END;
$$ LANGUAGE plpgsql;
