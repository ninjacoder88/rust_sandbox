`rustc [mainfile].rs`

`cargo new [project_name]`
creates a new project
a new project consists of a folder, a src directory in the project folder

`cargo build`
run from root directory
builds project and outputs to target/debug folder

`./target/debug/project_name.exe`

`cargo run`
run from root directory
builds and runs project

`cargo build --release`
builds project in release mode
takes a bit longer because of optimization

`cargo doc --open`
