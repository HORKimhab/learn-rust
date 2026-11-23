fn main() {
    println!("Hello variable. Running from ./bin/variable.rs");
    // let	Create a variable that cannot change
    let name = "HKimhab"; 
    // name = "HKimhab - update";
    println!("name = {name}");

    // let mut	Create a variable that can change
    let mut score = 0; 
    println!("score before change: {}", score);
    score = 23; 
    println!("score after change: {score}");

}
