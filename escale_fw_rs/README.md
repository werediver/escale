# eScale firmware in Rust

## UI structure

- Dashboard (weigh & time)
  - Taring
  - Calibrating

## General architecture and execution model

- Run _tasks_ in the order defined by a scheduler
- A task may need to initialize, can be ran, can terminate when done, can schedule other tasks
- Services
  - A task as a service can be discovered, can provide service-specific interface
    OR
  - A task can register (as) a service
  - A service can be resolved by other tasks
    (i.e. the service registry is separate from the task registry)
    OR
  - A message queue accessible to tasks instead?
    - (Weird, but) a service can be published via a message in the queue that is
      - never removed (a "pinned" message)
        OR
      - regularly re-posted

## UI

- The dynamic part of the view is (partially?) derived from the app state
- A view is rendered only when changed
- A view defines an input handling context
- A view is managed (?) by a task that also handles the relevant input

- Partial updates:
  - None: full redraw
    - No state needed
  - Two-stage: initial (empty form) draw and update draw (field values)
    - Simple state: uninitialized (a form draw needed), initialized (fields draw needed)
  - Full: redraw only what's needed

- Display API
  - A display interface is injected into a task as a dependency
    OR
  - A display interface is published as a "pinned" message in the message queue
    - The display service doesn't have to be a task
    - Con: rendering with the priority of the client
  - A client can publish a callback request in the message queue
    - Pro: rendering with the priority of the display service
    - Con: the client task may get control out of order
