type Byte = i8;
type Sstring = &'static str;

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
struct Pupil
{
    name: Sstring,
    last_name: Sstring,
    gender: Gender,
    age: Byte,
    grade: Byte,
}

pub fn main() 
{
    let mut _pupil: Vec<Pupil> = Vec::new();
    _pupil.push(
        Pupil {
            name: "John",
            last_name: "Doe",
            gender: Gender::Male,
            age: 16,
            grade: 10
        },
    );
    _pupil.push(
        Pupil {
            name: "Sarah liis",
            last_name: "Luha",
            gender: Gender::Female,
            age: 15,
            grade: 9
        },
    );

    for (i, pupil) in _pupil.iter().enumerate() {
        println!("Name : {:#?}, Last-name : {:#?}, Gender : {:#?}, Age : {:#?}, Grade : {:#?} --> at index {:#?}", 
        pupil.name, pupil.last_name, pupil.gender, pupil.age, pupil.grade, i);
    }

    print!("{:#?}", _pupil[0].name); 
}