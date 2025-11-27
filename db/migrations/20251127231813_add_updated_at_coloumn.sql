-- migrate:up
alter table rs_task_manager
add column updated_at timestamptz default now() not null;

-- migrate:down

