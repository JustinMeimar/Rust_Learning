use rocket::form::Form;


#[macro_export] //i think this allows macro defs to be accessable in entire project


fn index(cookies: &CookieJar<'_> -> Template) {
    let cookie = cookeis.get("message");

    Template::render("message", context! {
        message: cookie.map(|c| c.value()),
    })


}

