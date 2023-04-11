fn demo(){
    let x = five();
    println!("{x}");

    if x > 4 {
        println!("greater than 4");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    loop {
        println!("infinite");
    }
}

fn more_loops(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2l
        }
    };

    println!("The result is {result}");
}

fn five() -> i32 {
    5
}