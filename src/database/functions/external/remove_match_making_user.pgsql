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
 *     failure (group does not exist): 2
 *     failure (user is not added to group): 3
 */
CREATE OR REPLACE FUNCTION remove_match_making_user (
    discord_uuid TEXT,
    group_name TEXT
)
RETURNS INTEGER AS $$
DECLARE
    i BIGINT;
    user_id BIGINT;
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
    -- check if group exists
    IF NOT EXISTS (
        SELECT 1
          FROM match_making_groups mmg
         WHERE mmg.group_name = LOWER($2)
    )
    THEN
        RETURN 2;
    END IF;
    -- check if user and group combination already exists and is not subscribed
    IF EXISTS (
        SELECT 1
          FROM match_making_users mmu
         INNER JOIN users u ON mmu.user_id = u.user_id
         INNER JOIN match_making_groups mmg ON mmu.group_id = mmg.group_id
         WHERE u.discord_uuid = LOWER($1)
           AND mmg.group_name = LOWER($2)
           AND mmu.subscribed = FALSE
    )
    THEN
        RETURN 3;
    END IF;
    -- check if user and group combination already exists and is subscribed
    IF EXISTS (
        SELECT 1
          FROM match_making_users mmu
         INNER JOIN users u ON mmu.user_id = u.user_id
         INNER JOIN match_making_groups mmg ON mmu.group_id = mmg.group_id
         WHERE u.discord_uuid = LOWER($1)
           AND mmg.group_name = LOWER($2)
           AND mmu.subscribed = TRUE
    )
    -- unsubscribe them if so
    THEN
        UPDATE match_making_users mmu
           SET subscribed = FALSE
          FROM users u,
               match_making_groups mmg
         WHERE mmu.user_id = u.user_id
           AND mmu.group_id = mmg.group_id
           AND u.discord_uuid = LOWER($1)
           AND mmg.group_name = LOWER($2);
        RETURN 0;
    END IF;
END;
$$ LANGUAGE plpgsql;
