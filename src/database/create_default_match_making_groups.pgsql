-- create default match making groups
DO $$
BEGIN
    PERFORM add_match_making_group('1v1');
    PERFORM add_match_making_group('3v3');
    PERFORM add_match_making_group('6v6');
END
$$
