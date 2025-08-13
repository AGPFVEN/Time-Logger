\set ON_ERROR_STOP on

-- Simple assertions: fail the script if a condition is false
CREATE OR REPLACE FUNCTION assert_true(cond boolean, msg text)
RETURNS void AS $$
BEGIN
  IF NOT cond THEN
    RAISE EXCEPTION 'ASSERT TRUE failed: %', msg;
  END IF;
END; $$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION assert_eq_int(a bigint, b bigint, msg text)
RETURNS void AS $$
BEGIN
  IF a IS DISTINCT FROM b THEN
    RAISE EXCEPTION 'ASSERT EQ failed: % (got %, expected %)', msg, a, b;
  END IF;
END; $$ LANGUAGE plpgsql;

-- Expect a statement to throw, and the message to include expected_like (substring match)
CREATE OR REPLACE FUNCTION assert_throws(stmt text, expected_like text)
RETURNS void AS $$
DECLARE err text;
BEGIN
  EXECUTE stmt;
  RAISE EXCEPTION 'ASSERT THROWS failed: statement succeeded but should have thrown (expected: %)', expected_like;
EXCEPTION WHEN OTHERS THEN
  GET STACKED DIAGNOSTICS err = MESSAGE_TEXT;
  IF position(expected_like in err) = 0 THEN
    RAISE EXCEPTION 'ASSERT THROWS wrong error. expected like: %, got: %', expected_like, err;
  END IF;
END; $$ LANGUAGE plpgsql;