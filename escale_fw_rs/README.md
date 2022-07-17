# eScale firmware in Rust

## UI

- Dashboard
- Taring
- Calibrating

## General architecture and execution model

- Run _tasks_ in the order defined by a scheduler.
- A task may need to initialize, can be ran, can terminate when done, can schedule other tasks
- Services
  - A task as a service can be discovered, can provide service-specific interface
    OR
  - A task can register (as) a service
  - A service can be resolved by other tasks
    (i.e. the service registry is separate from the task registry)
    OR
  - A message queue accessible to tasks instead?

- List of tasks
- Iterate over the list of tasks
  - Run each task (priorities?)
