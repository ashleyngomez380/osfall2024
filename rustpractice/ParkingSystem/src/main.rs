
//define a struct Students with name and major

// We provide what kind of data it is going to have
#[derive(Debug)]
struct Student{
    name: String,
    major: String,
}
impl Student{
    fn new_student(name:String , major:String) -> Self{Student{
        name,
        major,
    }
}

    fn introduce_yourself(&self){
        println!("Hey my name is {} . I am majoring in {}",self.name,self.major)
    }
    fn change_major(&mut self , new_major:&str){
        self.major = new_major.to_string();
    }

}






//provide 3 methods
//Create a new student
//Change name
//Change majot




fn main() {
    let name ="Ashley".to_string();
    let major = "Computer Science".to_string();

    let mut s = Student::new_student(name,major);
    println!("{:?}",s);

    s.introduce_yourself();
    s.change_major("Artifical Intelligence");
    s.introduce_yourself();
}
