-- ===== Statuses =====
CREATE TABLE IF NOT EXISTS project_statuses (
  id        SMALLSERIAL PRIMARY KEY,
  key       TEXT NOT NULL UNIQUE,
  label     TEXT NOT NULL,
  is_closed BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS task_statuses (
  id        SMALLSERIAL PRIMARY KEY,
  key       TEXT NOT NULL UNIQUE,
  label     TEXT NOT NULL,
  is_closed BOOLEAN NOT NULL DEFAULT false
);

-- ===== Projects =====
CREATE TABLE IF NOT EXISTS projects (
  id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name        TEXT NOT NULL,
  description TEXT,
  status_id   SMALLINT REFERENCES project_statuses(id),
  finish_at   TIMESTAMPTZ,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ===== Tasks (adjacency model) =====
CREATE TABLE IF NOT EXISTS tasks (
  id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  project_id        UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  parent_id         UUID REFERENCES tasks(id) ON DELETE CASCADE,
  title             TEXT NOT NULL,
  description       TEXT,
  status_id         SMALLINT REFERENCES task_statuses(id),
  position          INT DEFAULT 1000,
  finish_at_initial TIMESTAMPTZ,
  finish_at         TIMESTAMPTZ,
  created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT tasks_not_self_parent CHECK (id <> parent_id)
);

-- Index to list tasks fast
-- CREATE INDEX IF NOT EXISTS idx_tasks_project_parent
  -- ON tasks(project_id, parent_id, position);

-- Enforce: parent & child belong to the same project
CREATE OR REPLACE FUNCTION ensure_same_project() RETURNS trigger AS $$
BEGIN
  IF NEW.parent_id IS NULL THEN
    RETURN NEW;
  END IF;

  IF NOT EXISTS (
    SELECT 1 FROM tasks p
    WHERE p.id = NEW.parent_id
      AND p.project_id = NEW.project_id
  ) THEN
    RAISE EXCEPTION 'Parent % must belong to the same project as the child', NEW.parent_id;
  END IF;

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_tasks_same_project ON tasks;
CREATE TRIGGER trg_tasks_same_project
BEFORE INSERT OR UPDATE OF parent_id, project_id ON tasks
FOR EACH ROW EXECUTE FUNCTION ensure_same_project();

-- Partial index for root tasks
--CREATE INDEX IF NOT EXISTS idx_tasks_roots
  --ON tasks(project_id, position)
  --WHERE parent_id IS NULL;

-- ===== Tags (With join table) =====
CREATE TABLE IF NOT EXISTS tags (
  id          SMALLSERIAL PRIMARY KEY,
  name        TEXT NOT NULL UNIQUE,
  color       TEXT NOT NULL CHECK (color ~* '^#([0-9a-f]{6})$'),
  description TEXT,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS project_tags (
  project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  tag_id     SMALLINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY (project_id, tag_id)
);

CREATE TABLE IF NOT EXISTS task_tags (
  task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  tag_id  SMALLINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY (task_id, tag_id)
);

-- ===== Time entries table =====
CREATE TABLE IF NOT EXISTS time_entries (
  id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  project_id       UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  task_id          UUID REFERENCES tasks(id) ON DELETE CASCADE,
  started_at       TIMESTAMPTZ NOT NULL,
  ended_at         TIMESTAMPTZ,
  description      TEXT,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),

  -- If ended_at is set, it must be after started_at
  CONSTRAINT te_valid_interval CHECK (
    ended_at IS NULL OR ended_at > started_at
  ),

  -- Computed duration (in seconds). Null while running (no ended_at yet).
  duration_seconds BIGINT GENERATED ALWAYS AS (
    CASE
      WHEN ended_at IS NULL THEN NULL
      ELSE (EXTRACT(EPOCH FROM (ended_at - started_at)))::BIGINT
    END
  ) STORED
);

-- ===== Project progress pages =====
CREATE TABLE IF NOT EXISTS project_progress_pages (
  id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  project_id       UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  title            TEXT NOT NULL,
  content_md       TEXT NOT NULL,
  progress_percent NUMERIC(5,2) CHECK (progress_percent >= 0 AND progress_percent <= 100),
  period_start     DATE,
  period_end       DATE,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at       TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Keep updated_at fresh
CREATE OR REPLACE FUNCTION touch_updated_at() RETURNS trigger AS $$
BEGIN
  NEW.updated_at := now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_ppp_touch ON project_progress_pages;
CREATE TRIGGER trg_ppp_touch
BEFORE UPDATE ON project_progress_pages
FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

---- Helpful indexes for listing & search
--CREATE INDEX IF NOT EXISTS idx_ppp_project_published
  --ON project_progress_pages(project_id, published_at DESC);

--CREATE INDEX IF NOT EXISTS idx_ppp_title_trgm
  --ON project_progress_pages USING gin (title gin_trgm_ops);

--CREATE INDEX IF NOT EXISTS idx_ppp_content_trgm
  --ON project_progress_pages USING gin (content_md gin_trgm_ops);

-- Ensure the task (if provided) belongs to the same project
CREATE OR REPLACE FUNCTION te_ensure_same_project() RETURNS trigger AS $$
BEGIN
  IF NEW.task_id IS NULL THEN
    RETURN NEW;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM tasks t
    WHERE t.id = NEW.task_id
      AND t.project_id = NEW.project_id
  ) THEN
    RAISE EXCEPTION 'Task % does not belong to project %', NEW.task_id, NEW.project_id;
  END IF;

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_te_same_project ON time_entries;
CREATE TRIGGER trg_te_same_project
BEFORE INSERT OR UPDATE OF project_id, task_id ON time_entries
FOR EACH ROW EXECUTE FUNCTION te_ensure_same_project();

-- Helpful indexes
-- CREATE INDEX IF NOT EXISTS idx_te_project_started ON time_entries(project_id, started_at);
-- CREATE INDEX IF NOT EXISTS idx_te_task_started    ON time_entries(task_id, started_at) WHERE task_id IS NOT NULL;
