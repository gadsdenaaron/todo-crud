#[macro_use] extern crate rocket;

use rocket::{serde::{Deserialize, json::Json, Serialize}, State};

use rocket::form::Form;
use rocket::fs::relative;
use rocket::response::{Flash, Redirect};
use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, context};

use tokio_postgres::{NoTls, Client, row::Row};

#[derive(Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
struct Task {
    id: i64,
    item: String
}

#[derive(FromForm)]
struct TaskItem{
    item: String
}

#[derive(FromForm)]
struct TaskID{
    id: i64
}

#[get("/")]
fn index() -> Flash<Redirect> {
    Flash::success(Redirect::to("/readtasks"), "")
}

#[get("/whoami")]
fn whoami() -> &'static str {
    "Aaron"
}

fn row_to_task(row: &Row) -> Task {
    Task {
        id: row.get::<&str, i64>("id"),
        item: row.get::<&str, String>("item")
    }
}

#[get("/readtasks")]
async fn read_tasks(client: &State<Client>) -> Template {

    let rows = client.query(
        "SELECT * FROM tasks",
        &[],
    ).await;

    let new_vec: Vec<_> = rows.as_ref().expect("postgre error").iter().map(row_to_task).collect();

    Template::render("main_table", context! { context: "value", m: new_vec })
}

#[get("/readtasks_as_json")]
async fn read_tasks_as_json(client: &State<Client>) -> Json<Vec<Task>> {

    let rows = client.query(
        "SELECT * FROM tasks",
        &[],
    ).await;

    let new_vec: Vec<_> = rows.as_ref().expect("postgre error").iter().map(row_to_task).collect();

    //let value = json!({"rows": new_vec});
    Json(new_vec)

}

#[post("/addtask", data = "<task_form>")]
async fn add_new_task(client: &State<Client>,  task_form: Form<TaskItem>) -> Flash<Redirect> {

    let task = task_form.into_inner();

    let params2: Vec<String> = vec![];

    let query = format!("INSERT INTO tasks (item) VALUES ('{}')", task.item);

    let query_result = client.execute_raw(&query, params2,).await;
    match query_result {

        Ok(v)  => {println!("updated {} rows", v); return Flash::success(Redirect::to("/readtasks"), "Todo successfully added.");}
        Err(e) => {println!("error {}", e); return Flash::success(Redirect::to("/readtasks"), "Error adding task");}
    }

}

#[get("/addtask")]
async fn add_task() -> Template {
    Template::render("main_form", context! { context: "value", id_form: false, item_form: true, page_submit: "addtask", action: "Add Task" })
}

#[post("/edittask", data = "<task_form>")]
async fn edit_task(client: &State<Client>, task_form: Form<Task>) -> Flash<Redirect> {

    let params2: Vec<String> = vec![];
    let task = task_form.into_inner();
    let query = format!("UPDATE tasks SET item = '{}' WHERE id = {} RETURNING *", task.item, task.id);
    
    let query_result = client.execute_raw(&query, params2,).await;
    match query_result {

        Ok(v)  => {println!("updated {} rows", v); return Flash::success(Redirect::to("/readtasks"), "Task edited succesfully");}
        Err(e) => {println!("error {}", e); return Flash::success(Redirect::to("/readtasks"), "Error Editing");}
    }
}

#[get("/edittask")]
async fn edit_task_page() -> Template {
    Template::render("main_form", context! { id_form: true, item_form: true, page_submit: "edittask", action: "Edit" })
}

#[post("/deletetask", data = "<task_form>")]
async fn delete_task(client: &State<Client>, task_form: Form<TaskID>) -> Flash<Redirect> {

    let id = task_form.into_inner().id;

    let params: Vec<String> = vec![];

    let query = format!("DELETE FROM tasks WHERE id = {} RETURNING *", id);

    let query_result = client.execute_raw(&query, params).await;
    match query_result {

        Ok(v)  => {println!("deleted {} rows", v); return Flash::success(Redirect::to("/readtasks"), "Task deleted succesfully");}
        Err(e) => {println!("error {}", e); return Flash::success(Redirect::to("/readtasks"), "Error Deleting");}
    }
}

#[get("/deletetask")]
async fn delete_task_page() -> Template {
    Template::render("main_form", context! { id_form: true, item_form: false, page_submit: "deletetask", action: "Delete" })
}

#[launch]
async fn rocket() -> _ {

    let Ok((client, connection)) =
        tokio_postgres::connect("host=localhost dbname=todo user=postgres password=postgres", NoTls).await else { panic!("You done fucked up"); };
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    rocket::build()
        .manage(client)
        .attach(Template::fairing())
        .mount("/", routes![index, whoami, add_task, read_tasks, edit_task, edit_task_page, delete_task, delete_task_page, add_new_task])
        .mount("/public", FileServer::from(relative!("/static")))
        
}