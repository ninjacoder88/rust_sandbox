fn demo() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'Z';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;

    let five_hundred = tup.0;
    let six_point_for = tup.1;
    let one = tup.2;

    let a = [1,2,3,4,5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let array = [i32; 5] = [1,2,3,4,5];
    let all_threes = [3; 5]; //[3,3,3,3,3]

    let first = a[0];
    let second = a[1];
}