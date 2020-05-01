/*
 * match making users table
 *
 * table containing information linking users and match making groups
 * they belong to.
 *
 * columns:
 *     user_id: user id referenced from users table
 *     group_id: group id referenced from match making groups table
 *     subscribed: whether or not the user is currently subscribed the group
 */
CREATE TABLE IF NOT EXISTS match_making_users (
    user_id BIGINT NOT NULL REFERENCES users ON DELETE CASCADE,
    group_id BIGINT NOT NULL REFERENCES match_making_groups,
    subscribed BOOLEAN NOT NULL,
    PRIMARY KEY (user_id, group_id)
);
