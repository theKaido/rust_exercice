// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// I AM DONE

//macro_rules! my_macro; this function doesnt work se renseigner
//sur les prototypes des fonctions en rust

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}


fn main() {
    my_macro!();
}

