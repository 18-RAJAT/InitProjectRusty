// use ferris_says::say;
// use std::io::{stdout,BufWriter};

// fn main()
// {
//     let stdout=stdout();
//     let message=String::from("Hello Rust");
//     let width=message.chars().count();

//     let mut writer=BufWriter::new(stdout.lock());
//     say(message.as_bytes(),width,&mut writer).unwrap();
// }







//Binding

// fn main()
// {
//     let x:i32;//type
//     println!("Success");

//     mutable
//     let mut x:i32;
//     x=1;
//     x+=2;
//     assert_eq!(x,3);
//     println!("Success");
// }






// Shadowing

fn main()
{
    let x: i32=10;
    {
        let y: i32=20;
        println!("The value of x is {} and the value of y is {}",x,y)
    }
}