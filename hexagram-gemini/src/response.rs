#[derive(Debug)]
enum Response {
    Input(),
    Success(),
    Redirect(),
    Tempfail(),
    Permfail(),
    Auth(),
}
