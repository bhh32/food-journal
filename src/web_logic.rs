use rocket::{
    response::Redirect,
    Request,
};

use rocket_dyn_templates::{Template, handlebars, context};

#[get("/")]
pub fn index() -> Template {
   Template::render("index", context! {
       title: "Food Journal",
   })
}
