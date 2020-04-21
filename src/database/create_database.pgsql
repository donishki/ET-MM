-- create et_mm_db database if it does not already exist
DO $$
  BEGIN
    CREATE DATABASE et_mm_db OWNER et_mm;
    EXCEPTION WHEN DUPLICATE_OBJECT THEN
      RAISE NOITCE 'Not creating role: et_mm - it already exists.';
    GRANT CONNECT ON DATABASE et_mm_db TO et_mm; 
  END
$$