/*
 * match making groups table
 *
 * table containing information for different match making groups.
 * the default match making groups are 1v1, 3v3, and 6v6. More can
 * be added by using the 'add_match_making_group' database function.
 *
 * columns:
 *     group_id: unique database group id for relational purposes
 *     group_name: unique discord group name
 */
CREATE TABLE IF NOT EXISTS match_making_groups (
    group_id SERIAL PRIMARY KEY,
    group_name TEXT UNIQUE NOT NULL
);
