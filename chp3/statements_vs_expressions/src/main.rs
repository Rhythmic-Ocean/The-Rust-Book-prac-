//statements vs expressions
//statements ends with ;, expressions don't
//expressions have a return value after some evaluation, but statements don't, they only perform a task with no return value

fn main(){
    let x = 6; //the entire thing is an statement, while just 6 is an expression
    //let x = (let y = 6); //error cuz statements don't return a value, the error will say "expected expression"
    let y = {
        let x = 3;
        x + 1;
    };
    println!("{:?}", y);
}