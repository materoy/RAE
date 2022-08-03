
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/application.proto")?;

    // println!("Building example test binaries ...");

    // let samples_dir = "src/samples";
    // let samples = std::fs::read_dir(samples_dir)
    //     .expect(format!("Problem opening dir {}", samples_dir).as_str());

    // for sample in samples {
    //     let path = sample.unwrap().path();
    //     let cd = Command::new("cd");
    //     cd.arg(path).spawn().unwrap();

    //     // Build example binaries
    //     let mut build_command = Command::new("cargo");

    //     match build_command.args(vec!["build"]).status() {
    //         Ok(output) => {
    //             if output.success() {
    //                 println!("Process completed successfully");
    //             } else {
    //                 eprintln!("Process did not finish gracefully please build manually");
    //             }
    //         }
    //         Err(err) => {
    //             eprintln!("Build error: {}", err);
    //         }
    //     }

    //     cd.arg("..").spawn().unwrap();
    // }
    Ok(())
}
