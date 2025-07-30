use rust_website_gen::App;
use std::fs;

fn main() {
    App::new()
        .route("/", root())
        .route("/posts", posts())
        .build("output")
        .unwrap();
}

fn root() -> &'static str {
    "hello world"
}

fn posts() -> App {
    let mut app = App::new();
    app.route("/", "hello blog");

    for entry in fs::read_dir("posts").expect("failed to read posts") {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_file() {
            panic!("all entries inside posts must be files");
        }

        let content = fs::read_to_string(&path).unwrap();
        let name = path.file_stem().unwrap().display();
        app.route(format!("/{name}"), content);
    }

    app
}
