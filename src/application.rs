#[allow(dead_code)]

pub struct Application<'a> {
    name: String,
    executable: &'a [u8],
    execute_command: String,
    data: String,
    path: String,
    argv: Vec<String>,
    envv: Vec<String>,
}

// impl<'a> Application<'a> {
//     fn new(
//         name: String,
//         executable: &'a [u8],
//         execute_command: String,
//         data: String,
//         path: String,
//         argv: Vec<String>,
//         envv: Vec<String>,
//     ) -> Self {
//         Application {
//             name,
//             executable,
//             execute_command,
//             data,
//             path,
//             argv,
//             envv,
//         }
//     }
// }
