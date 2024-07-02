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




//mut means->it is a keyword allows to explicitly specify which variables you intend to change,making your code clearer and helping prevent accidental modifications of variables that should remain constant.



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







// fn main()
// {
//     let x: i32=10;
//     {
//         let y: i32=20;
//         println!("The value of x is {} and the value of y is {}",x,y)
//     }
// }



//Scope(Multiple Functions)

// fn main()
// {
//     x_value();
// }

// fn x_value()
// {
//     let x="hello";
//     println!("{},world",x);
// }




//Shadowing

// fn main()
// {
//     let x:i32=4;
//     let x="Some";//overwrite(changing the type)
//     println!("{} value of x",x);
// }



//Example of Shadowing:

// fn main()
// {
    //     let x:i32=5;
    //     {
//         let x=15;
//         assert_eq!(x,15);
//     }
//     assert_eq!(x,5);
//     let x=30;
//     println!("{} value of x",x);
// }





//Example of Shadowing:

fn main()
{
    let mut x:i32=1;
    x=7;
    let mut x=4;
    x+=3;
    println!("Working");
}