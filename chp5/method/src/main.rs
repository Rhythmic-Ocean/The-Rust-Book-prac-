#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle{
    fn area(&self) -> i32{ //we use &self instead of self because we want the method to only borrow the instance
        //BUT in some cases we might want do to self, specially if we don't want the main to change anything, hence changing ownership
        self.width*self.height
    }

    fn can_fit(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    fn square(size: i32) -> Rectangle{ //This is associate function, it's related to the struct itself
        Rectangle{
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 55,
        height: 45
    };

    let rect2 = Rectangle{
        width: 66,
        height:32
    };
    let area = rect1.area(); //to call a method on an instance use .method()

    let sq = Rectangle::square(5); //to call and associate function! Use ::
    println!("The square is: {:?}", sq);

    if rect1.can_fit(&rect2){
        println!("Rect 1 can fit rect 2");
    }
    else{
        println!("Rect1 cannot fit rect2");
    }

    println!("The area or rect1 is {}", area);


}

//try using impl method instead of function, if you want a method to be function to be a struct specific
