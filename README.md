# Krabby Do

Authors: [Rohan Singh](mailto:rohsingh@pdx.edu), [Kajal Patil](mailto:kajal@pdx.edu), [Prachi Kashyap](mailto:pk23@pdx.edu)

---

## Overview

Krabby Do is an application based in Rust that allows a user to create to-do lists and event reminders to help them manage their tasks and events.

---

## Intended functions

- Create a new task.
- Edit an existing task.
- Delete an existing task.
- Mark a task as complete or incomplete.
- Assign deadlines to a certain task.
- Receive a notification for a particular task at a time specified by the user.

---

## Build Instructions

1. Install Rust in your system if you don't have it installed. It is available here: https://www.rust-lang.org/tools/install
2. Clone the repository from either of the following links:
   - https://gitlab.cecs.pdx.edu/krabbydo/krabbydo.git (HTTPS)
   - git@gitlab.cecs.pdx.edu:krabbydo/krabbydo.git (SSH)
3. Navigate to the repository.
4. Open terminal and run the following command:

```sh
cargo run
```

---

## Testing

Testing was done using unit tests embedded into the code files in each crate.

---

## Example

[How to use Krabby Do](./resources/usage_example.md)

---

## Analysis

### What Worked

- Rohan - Creating UI elements and implementing layouts for them.
- Kajal -
- Prachi -

### What Didn't Work

- Rohan - Styling the UI elements.
- Kajal -
- Prachi -

### Conclusion

- Rohan - I am satisfied with the result but in future I would like to improve the styling of the application as currently it looks a bit too bland for my taste.
- Kajal -
- Prachi -

---

## License

[LICENSE](./LICENSE)

---

## Project Structure

Krabby Do consists of three crates with each crate created and managed by a member of the team:

- #### UI

  This crate enables a user to add tasks or events for which the user needs to create reminders through a GUI. The UI offers CRUD options such as create, view, edit and delete a task. The UI implementation has been done using **egui**.

  This crate is handled by Rohan Singh.

- #### Middleware

  This component will connect with the database and will reflect the changes done by the user to the database. As a standard practice, the delete option will update the active indicator to ‘No’ from ‘Yes’. Middleware will consist of four APIs: create/update/fetch/delete . This component will form a library crate which can be utilized by the UI as well as the Notification Crate.

  This crate is handled by Kajal Patil.

- #### Notifications

  This component will fetch data based on date and send the notifications to the user. We will use the ‘notify-rust’ library for implementation. For each task, a separate notification will be sent.

  This crate is handled by Prachi Kashyap.

---

### Repository

[KrabbyDo](https://gitlab.cecs.pdx.edu/krabbydo/krabbydo)

Mirrored on: [Krabby Do - Rohan's GitHub](https://github.com/rohan-singh1/KrabbyDo)

---
