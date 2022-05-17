
pub struct Application<'a> {
    name: String,
    executable: &'a [u8],
    execute_command: String,
    data: String,
    path: String,
    argv: Vec<String>,
    envv: Vec<String>,
}
