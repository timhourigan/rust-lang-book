fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // addition
    let sum = 5 + 10;
    println!("The value of sum is {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is {}", remainder);

    // Extras
    // division
    let quotient = 7 / 2;
    println!("The value of quotient is {}", quotient);
    // remainder
    let remainder = 7 % 2;
    println!("The value of remainder is {}", remainder);

    let f: bool = false;
    println!("The value of f is {}", f);

    let c = 'c';
    println!("The value of c is {}", c);

    let z = 'Z';
    println!("The value of z is {}", z);

    // Tuples
    let tup = (500, 6.4, 1);
    let (xx, yy, zz) = tup;
    println!("The values are {}, {}, {}", xx, yy, zz);

    let tup_mix: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup_mix is {:?}", tup_mix);
    println!("The first value is {}", tup_mix.0);

    // Arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is {:?}", a);

    let aa = [3; 5];
    println!("The value of aa is {:?}", aa);
    let first = a[0];
    println!("The value of first is {:?}", first);
}
