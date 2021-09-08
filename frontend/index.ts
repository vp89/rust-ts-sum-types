import axios from 'axios';

(async () => {
    const response = (await axios.get<Animal>('http://localhost:4000/')).data;

    switch (response.animal.type) {
        case AnimalKind.Dog:
            console.log(`${response.animal.a} ${response.animal.b}`);
            break;
        case AnimalKind.Cat:
            console.log(`${response.animal.c} ${response.animal.d}`);
            break;
        case AnimalKind.Chicken:
            console.log(`${response.animal.a} ${response.animal.b}`);
            break
        default:
            exhaustive(response.animal);
    }
})();

function exhaustive(x: never): never {
    throw new Error("Unhandled animal kind in the system");
}

type Animal = {
    animal_id: number,
    animal: Dog | Cat | Chicken
}

enum AnimalKind {
    Dog = "Dog",
    Cat = "Cat",
    Chicken = "Chicken"
}

type Dog = {
    type: AnimalKind.Dog,
    a: string,
    b: string
}

type Cat = {
    type: AnimalKind.Cat,
    c: string,
    d: string
}

type Chicken = {
    type: AnimalKind.Chicken,
    a: string,
    b: string
}
