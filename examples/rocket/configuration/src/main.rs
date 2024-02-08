#[macro_use]
extern crate rocket;



#[get("/")]
fn index() -> &'static str {
    rocket::info!("profile is debug: {:?}", rocket::Config::default().profile == "debug");

    let custom_a: String = rocket::Config::figment().extract_inner("custom_a").unwrap_or(String::from("some default"));
    rocket::info!("custom_a {:?}", custom_a);

    let custom_b = rocket::Config::figment().extract_inner::<String>("custom_b").unwrap_or(String::from("some default"));
    rocket::info!("custom_b {:?}", custom_b);

    let custom_in_default: String = rocket::Config::figment().extract_inner("custom_in_default").unwrap_or(String::from("some other default"));
    rocket::info!("custom_in_default {:?}", custom_in_default);

    rocket::info!("------");
    rocket::info!("default: {:#?}", rocket::Config::default());

    "See the console"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn home() {
        let client = Client::tracked(super::rocket()).unwrap();
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}
