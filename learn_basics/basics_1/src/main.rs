fn vec_string(){
/* 
 * diagram from https://blog.thoughtram.io/ownership-in-rust/
 *
            [–– names ––]
            +–––+–––+–––+
stack frame │ • │ 3 │ 2 │
            +–│–+–––+–––+
              │
            [–│–– 0 –––] [–––– 1 ––––]
            +–V–+–––+–––+–––+––––+–––+–––+–––+
       heap │ • │ 8 │ 6 │ • │ 12 │ 9 │       │
            +–│–+–––+–––+–│–+––––+–––+–––+–––+
              │\   \   \  │
              │ \   \    length
              │  \    capacity
              │    buffer │
              │           │
            +–V–+–––+–––+–––+–––+–––+–––+–––+
            │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+
                          │
                          │
                        +–V–+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
                        │ C │ h │ r │ i │ s │ t │ o │ p │ h │   │   │   │
                        +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
*/
    let names = vec!["Pascal".to_string(), "Chirstoph".to_string()];
    /* If names goes out of scope, then its values will be dropped, which cascades
     * and eventually drops the pointers for "Pascal" and "Christoph"
     * */
}

fn greeting(){
    let s = "Have a nice day".to_string();
    
    println!("{}", s);
}
/*
 * improved with reference to duplicate value (parameter in a function)
 * */
fn greet(name: &String){
    //greet "borrows" name, but does not own it. Rust ownership: A value is only owned by one
    //variable.
    println!("Hello, {}!", name);
}

fn main() {
    let name = "Lisp".to_string(); 
    greet(&name);
    greet(&name);
    /*
     *rather than greet(name);
     *            greet(name);
     * */
}
