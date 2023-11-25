# Rust CRUD

Example TODO CRUD app written in Rust

## Description
Let people know what your project can do specifically. Provide context and add a link to any reference visitors might be unfamiliar with. A list of Features or a Background subsection can also be added here. If there are alternatives to your project, this is a good place to list differentiating factors.

This is a CRUD app written in rust using the web framework [Rocket](https://rocket.rs/), a postgres database and the [tokio-postgres](https://docs.rs/tokio-postgres/latest/tokio_postgres/) library for db integration. It is based on [this](https://betterprogramming.pub/how-to-write-a-web-app-in-rust-part-2-2da195369fc1) tutorial for inspiration although I wouldnt call the code or functionality identical.

The basic functions of this app is:
 - list current tasks
 - add a new task
 - edit an existing task
 - remove an existing task


## Installation
As this is an example project the installation is not that simple

- 1. clone this repo
- 2. install a postgres instance
- 3. modify the code to connect to your postgres instance
- 4. Create the necessary table in postgres todo.tasks columns (id: BIGSERIAL, item: TEXT)
- 5. change into this directory and cargo run
- 6. hopefully connect to the running web server. 

