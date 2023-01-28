use std::io;



struct Todo{
    active: bool,
    todo: String,
    time: String,
}

fn main() {
    let mut todos: [Todo;100]; 
    loop {
        let mut choice = String::new();
        println!("Choose\n 1.Add\n 2.Delete\n 3.Exit");
        io::stdin().read_line(&mut choice).expect("error");
        let num: i32 = choice.trim().parse().expect("Not an i32");
        if num == 3 {
            println!("Exit");
            break;
        } else if num == 1{
            add_todo(&mut todos); //right here
        }
    }
}

fn add_todo(_todos: &mut [Todo] ) {
    let mut note = String::new();
    println!("Enter your note : ");
    io::stdin().read_line(&mut note).expect("err");
}