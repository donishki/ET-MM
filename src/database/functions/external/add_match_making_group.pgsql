/*
 * add a new matchmaking group entry
 *
 * args:
 *    group_name: match making group name
 *
 * returns:
 *     success: 0
 *     failure (group already exists): 1
 */
CREATE OR REPLACE FUNCTION add_match_making_group (
    group_name TEXT
)
RETURNS INTEGER AS $$
BEGIN
    -- check if match making group already exists
    IF EXISTS (
        SELECT 1
          FROM match_making_groups mmg
         WHERE mmg.group_name = LOWER($1)
    )
    THEN
        RETURN 1;
    END IF;
    -- insert values into table
    INSERT INTO match_making_groups (group_name)
        VALUES (LOWER($1));
    RETURN 0;
END;
$$ LANGUAGE plpgsql;
