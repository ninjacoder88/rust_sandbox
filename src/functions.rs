fn fn_demo(){
    let x = five();
    println!("{x}");

    another_function();
}

fn five() -> i32 {
    5
}

fn another_function(x: i32){
    println!("another function {x}");
}