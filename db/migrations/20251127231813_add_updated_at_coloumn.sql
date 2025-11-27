-- migrate:up
-- Step 1: Add column
alter table rs_task_manager
add column updated_at timestamptz;

-- Step 2: Set updated_at to created_at for all existing rows
update rs_task_manager
set updated_at = created_at
where updated_at is null;

-- migrate:down 

