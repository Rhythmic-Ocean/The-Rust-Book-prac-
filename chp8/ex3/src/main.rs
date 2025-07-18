use std::collections::HashMap;
use std::io;

fn main() {
    let mut lo= true;

    let mut company = HashMap::new();
    while lo{
        println!("Enter a dept name to exit!");
        println!("Enter your command!");

        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read at input!");


    let mut words:Vec<String> = Vec::new();

        for i in command.split_whitespace(){
            let p = i.to_string();
            words.push(p);
        }


        if words[0] == "Add".to_string(){
            let dept = &words[3];
            let person = &words[1];
            let elem = company.entry(dept.to_string()).or_insert(vec![]);
            (*elem).push(person.to_string());
        }
        else{
            let dept = &words[0];
            let elem = company.entry(dept.to_string()).or_insert(vec![]);
            (*elem).sort();
            println!("{:?}", company);
            lo = false;
        }

    }


}

