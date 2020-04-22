-- create default match making groups
DO $$
BEGIN
    SELECT add_match_making_group('1v1');
    SELECT add_match_making_group('3v3');
    SELECT add_match_making_group('6v6');
END
$$
