-- ==============================================================================
-- SnowIDv2 Pure PostgreSQL Implementation (for Cloud / Managed DBs)
-- Compatible with AWS RDS, Supabase, Neon, GCP Cloud SQL, and standard Postgres
-- ==============================================================================

-- 1. Create a sequence for the 14-bit sequence portion (0 to 16383)
CREATE SEQUENCE IF NOT EXISTS snowidv2_seq
    MINVALUE 0
    MAXVALUE 16383
    CYCLE;

-- 2. Create the standalone SnowIDv2 generation function
-- Parameters:
--   machine_id: Worker / Node ID (0 to 63, default 1)
-- Returns:
--   64-bit Snowflake-compatible integer ID (BIGINT)
CREATE OR REPLACE FUNCTION snowidv2_next(machine_id INT DEFAULT 1)
RETURNS BIGINT
LANGUAGE plpgsql
VOLATILE
AS $$
DECLARE
    custom_epoch BIGINT := 1700000000000; -- Milliseconds since UNIX epoch
    now_ms BIGINT;
    seq_val BIGINT;
    result_id BIGINT;
BEGIN
    IF machine_id < 0 OR machine_id > 63 THEN
        RAISE EXCEPTION 'machine_id % is out of range (must be between 0 and 63)', machine_id;
    END IF;

    now_ms := (EXTRACT(EPOCH FROM clock_timestamp()) * 1000)::BIGINT - custom_epoch;
    seq_val := nextval('snowidv2_seq');

    -- Build 64-bit ID:
    -- [ 44 bits timestamp ms ] [ 6 bits machine_id ] [ 14 bits sequence ]
    result_id := (now_ms << 20) | (machine_id::BIGINT << 14) | seq_val;

    RETURN result_id;
END;
$$;

-- 3. Create a decoder function to inspect IDs directly in SQL queries
CREATE OR REPLACE FUNCTION snowidv2_decode(input_id BIGINT)
RETURNS TABLE (
    timestamp_ms BIGINT,
    generated_at TIMESTAMPTZ,
    machine_id INT,
    sequence INT
)
LANGUAGE plpgsql
IMMUTABLE
AS $$
DECLARE
    custom_epoch BIGINT := 1700000000000;
    ts_ms BIGINT;
BEGIN
    ts_ms := (input_id >> 20) + custom_epoch;
    timestamp_ms := ts_ms;
    generated_at := TO_TIMESTAMP(ts_ms / 1000.0);
    machine_id := ((input_id >> 14) & 63)::INT;
    sequence := (input_id & 16383)::INT;
    RETURN NEXT;
END;
$$;
