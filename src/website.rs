use crate::database;
use crate::models;
use crate::rocket;

use rocket::request::Form;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::Template;

#[database("sqlite_main")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn index(conn: DbConn) -> Template {
    let mut context = Context::new();
    context.insert("title", "Rusty Shop");
    let products_to_show = database::get_all_products(&conn, 20);
    context.insert("products", &products_to_show);
    context.insert("page", "index");
    Template::render("index", &context)
}

#[get("/dashboard")]
fn dashboard(conn: DbConn) -> Template {
    let mut context = Context::new();
    context.insert("title", "Dashboard");
    context.insert("page", "dashboard");
    Template::render("dashboard", &context)
}

#[get("/checkout")]
fn checkout(conn: DbConn) -> Template {
    let mut context = Context::new();
    context.insert("title", "Checkout");
    context.insert("page", "checkout");
    Template::render("checkout", &context)
}

#[post("/signup", data = "<user>")]
fn signup(conn: DbConn, user: Form<models::FormUser>) {
    database::insert_user(&conn, user.into_inner());
}

pub fn run_rocket() {
    rocket::ignite()
        .mount("/", routes![index, dashboard, checkout, signup])
        .mount(
            "/assets",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/files")),
        )
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .launch();
}
