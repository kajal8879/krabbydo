# Krabby Do

Authors: [Rohan Singh](mailto:rohsingh@pdx.edu), [Kajal Patil](mailto:kajal@pdx.edu), [Prachi Kashyap](mailto:pk23@pdx.edu)

---

### Overview

Krabby Do is an application based in Rust that allows a user to create to-do lists to help them manage their tasks.

Krabby Do will offer the following features:

- Create a new task.
- Edit an existing task.
- Delete an existing task.
- Mark a task as complete or incomplete.
- Assign deadlines to a certain task.
- Receive a notification for a particular task at a time specified by the user.

This project will consist of three major parts:

- #### UI

  This component will enable a user to add tasks for which he/she needs to create a reminder. The UI will offer CRUD options such as create, edit and delete a task. The UI implementation will be done using egui. This component will form one of the three crates for the project.

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
