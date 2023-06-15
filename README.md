# Krabby Do

Authors: [Rohan Singh](mailto:rohsingh@pdx.edu), [Kajal Patil](mailto:kajal@pdx.edu), [Prachi Kashyap](mailto:pk23@pdx.edu)

## Overview

Krabby Do is an application based in Rust that allows a user to create to-do lists and event reminders to help them manage their tasks and events.

## Intended functions

- Create a new task.
- Edit an existing task.
- Delete an existing task.
- Mark a task as complete or incomplete.
- Assign deadlines to a certain task.
- Receive a notification for a particular task at a time specified by the user.

## Build Instructions

1. Install Rust in your system if you don't have it installed. It is available here: https://www.rust-lang.org/tools/install
2. Install MongoDB Community Edition from here: https://www.mongodb.com/docs/manual/administration/install-community/
3. Install MongoDB Compass from here: https://www.mongodb.com/try/download/compass
4. Clone the repository from either of the following links:
   - https://gitlab.cecs.pdx.edu/krabbydo/krabbydo.git (HTTPS)
   - git@gitlab.cecs.pdx.edu:krabbydo/krabbydo.git (SSH)
5. Navigate to the repository.
6. Open terminal and run the following command:

```sh
cargo run
```

7. To view the entries in the database:
   1. Open MongoDB Compass and connect to the following URI: `mongodb://localhost:27017/`
   2. Go to the database named **_events_**.
   3. Go to the document named **_todos_**, the event entries are listed there.

## Testing

Testing was done using unit tests embedded into the code files in each crate.

## Example

[How to use Krabby Do](./resources/usage_example.md)

## Analysis

### What Worked

- Rohan - Creating UI elements and implementing layouts for them.
- Kajal - Created middleware section which interacts with mongoDB. Also Added notification crate which sends out notifications as part of the MVP criteria.
- Prachi - Implementation of Searching, Sorting, Exporting and adding tags to the events

### What Didn't Work

- Rohan - Styling the UI elements.
- Kajal - Making the fields in eventEntry structs optional; currently all the fields are mandatory. Also, for notification crate we couldn't implement a background user agent.
- Prachi - Notifications (My Macbook was throwing some errors while implementing notifications, the notification was not going through the Notification Center).

### Conclusion

- Rohan - I am satisfied with the result but in future I would like to improve the styling of the application as currently it looks a bit too bland for my taste.
- Kajal - Establishing MongoDB connection was the tricky part. And dealing with data was difficult as well. Faced so many parsing issues, but it was a great learning experience. now I feel confident with data types and parsing. Finding a proper crate which will work for you was difficult, as I feel there is lack of documentation for rust.
- Prachi - There are many things to figure out about the libraries and their usage. But after working on it, I am happy that I got to learn a new programming language.

## License

[LICENSE](./LICENSE)

## Project Structure

Krabby Do consists of three crates with each crate created and managed by a member of the team.

- #### UI

  This crate enables a user to add tasks or events for which the user needs to create reminders through a GUI. The UI offers CRUD options such as create, view, edit and delete a task. The UI implementation has been done using **egui**.

  This crate is handled by Rohan Singh.

- #### Middleware

  This component connects with the database and changes done by the user reflects in database. As a standard practice, the delete option updates the active indicator to ‘No’ from ‘Yes’. Middleware consists of six APIs: create/update/fetch all tasks/ fetch todays tasks/delete/ mark as done . This component is a library crate which is utilized by the UI as well as the Notification Crate.

  This crate is handled by Kajal Patil.

- #### Notifications

  This component fetches data based on date and send the notifications to the user. We use the ‘notify-rust’ library for implementation. For each task, a separate notification is sent.

  This crate is handled by Kajal Patil/Prachi Kashyap.

## Repository

[KrabbyDo](https://gitlab.cecs.pdx.edu/krabbydo/krabbydo)

Mirrored on: [Krabby Do - Rohan's GitHub](https://github.com/rohan-singh1/KrabbyDo)

---
