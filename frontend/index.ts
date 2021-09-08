import axios from 'axios';

(async () => {
    const animal = (await axios.get<Animal>('http://localhost:4000/')).data;

    switch (animal.animal.type) {
        case "Dog":
            const dog = animal.animal as Dog;
            console.log(`${dog.a} ${dog.b}`);
            break;
        case "Cat":
            const cat = animal.animal as Cat;
            console.log(`${cat.c} ${cat.d}`);
            break;
        case "Chicken":
            const chicken = animal.animal as Chicken;
            console.log(`${chicken.a} ${chicken.b}`);
            break
        default:
            exhaustive(animal.animal);
    }
})();

function exhaustive(x: never): never {
    throw new Error("BLARGH");
}

type Animal = {
    animal_id: number,
    animal: Dog | Cat | Chicken
}

type Dog = {
    type: "Dog",
    a: string,
    b: string
}

type Cat = {
    type: "Cat",
    c: string,
    d: string
}

type Chicken = {
    type: "Chicken",
    a: string,
    b: string
}
