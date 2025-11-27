-- migrate:up
create type task_status as enum (
  'pending',
  'in_progress',
  'completed',
  'archived'
);

create table rs_task_manager
(
    id uuid default gen_random_uuid() not null constraint rs_task_manager_pk primary key,
    description text not null check (description != ''),
    status task_status default 'pending' not null,
    created_at timestamptz default now() not null,
    flagged bool default false not null
);

-- migrate:down