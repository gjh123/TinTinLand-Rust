enum Animals {
    AnimalDog(Dog),
    AnimalCat(Cat),
    AnimalBird(Bird),
}
trait Animal {
    fn call(&self);
}

struct Dog;
struct Cat;
struct Bird;

impl Animal for Dog {
    fn call(&self) {
        println!("Dog: 汪汪");
    }
}

impl Animal for Cat {
    fn call(&self) {
        println!("Cat: 喵喵");
    }
}

impl Animal for Bird {
    fn call(&self) {
        println!("Bird: 叽叽喳喳");
    }
}

impl Animal for Animals {
    fn call(&self) {
        match self {
            Animals::AnimalDog(dog) => dog.call(),
            Animals::AnimalCat(cat) => cat.call(),
            Animals::AnimalBird(bird) => bird.call(),
        }
    }
}

pub fn run() {
    let vec: Vec<Box<dyn Animal>> = vec![
        Box::new(Animals::AnimalDog(Dog)),
        Box::new(Animals::AnimalCat(Cat)),
        Box::new(Animals::AnimalBird(Bird)),
    ];

    for item in vec {
        item.call();
    }
}
