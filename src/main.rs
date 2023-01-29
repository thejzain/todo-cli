// use std::io;

// struct Todo {
//     active: bool,
//     todo: String,
// }

// fn main() {
//     let mut todos: Vec<Todo> = Vec::new();
//     loop {
//         let mut todos = &mut todos;
//         let mut choice = String::new();
//         println!("Choose\n 1.Add\n 2.Delete\n 3.Exit");
//         io::stdin().read_line(&mut choice).expect("error");
//         let num: i32 = choice.trim().parse().expect("Not an i32");
//         if num == 4 {
//             println!("Exit");
//             break;
//         } else if num == 1 {
//             let mut todos = add_todo(&mut todos);
//             for x in todos {
//                 println!("{}", x.todo);
//             }
//         } else if num == 3 {

//         }else {
//             println!("Invalid choice!!");
//         }
//     }
// }

// fn add_todo(_todos: &mut Vec<Todo>) -> &mut Vec<Todo> {
//     let mut note = String::new();
//     println!("Enter your note : ");
//     io::stdin().read_line(&mut note).expect("err");
//     let todo = Todo {
//         active: true,
//         todo: String::from(note),
//     };
//     _todos.push(todo);
//     return _todos;
// }


fn main(){
    
}