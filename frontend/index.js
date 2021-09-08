"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const axios_1 = __importDefault(require("axios"));
(async () => {
    const animal = (await axios_1.default.get('http://localhost:4000/')).data;
    switch (animal.animal.type) {
        case "Dog":
            const dog = animal.animal;
            console.log(`${dog.a} ${dog.b}`);
            break;
        case "Cat":
            const cat = animal.animal;
            console.log(`${cat.c} ${cat.d}`);
            break;
        case "Chicken":
            const chicken = animal.animal;
            console.log(`${chicken.a} ${chicken.b}`);
            break;
        default:
            exhaustive(animal.animal);
    }
})();
function exhaustive(x) {
    throw new Error("BLARGH");
}
