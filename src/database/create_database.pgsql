-- create et_mm_db database if it does not already exist
SELECT 'CREATE DATABASE et_mm OWNER et_mm'
	WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'et_mm')\gexec

-- grant connection rights to et_mm for peer mapping
GRANT CONNECT ON DATABASE et_mm TO et_mm;
