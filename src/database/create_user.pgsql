-- create et_mm user if it does not already exist
DO $$
BEGIN
	CREATE ROLE et_mm LOGIN;
	EXCEPTION WHEN DUPLICATE_OBJECT THEN
		RAISE NOTICE 'Not creating role: et_mm - it already exists.';
END
$$
