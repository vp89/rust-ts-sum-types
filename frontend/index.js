"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const axios_1 = __importDefault(require("axios"));
(async () => {
    const response = (await axios_1.default.get('http://localhost:4000/')).data;
    switch (response.animal.type) {
        case AnimalKind.Dog:
            console.log(`${response.animal.a} ${response.animal.b}`);
            break;
        case AnimalKind.Cat:
            console.log(`${response.animal.c} ${response.animal.d}`);
            break;
        case AnimalKind.Chicken:
            console.log(`${response.animal.a} ${response.animal.b}`);
            break;
        default:
            exhaustive(response.animal);
    }
})();
function exhaustive(x) {
    throw new Error("BLARGH");
}
var AnimalKind;
(function (AnimalKind) {
    AnimalKind["Dog"] = "Dog";
    AnimalKind["Cat"] = "Cat";
    AnimalKind["Chicken"] = "Chicken";
})(AnimalKind || (AnimalKind = {}));
