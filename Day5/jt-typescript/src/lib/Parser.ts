import Crate from "./Crate";
import Instruction from "./Instruction";
import Stack from "./Stack";

export default class Parser{
    public instructions: Instruction[];
    public stacks: Stack[];
    constructor(private _input: string){
        const [stackInput, instructionInput] = this.separateCratesAndInstructions(this._input);
        this.stacks = this.createStackArrays(stackInput);
        this.instructions = this.getInstructions(instructionInput);
    }

    separateCratesAndInstructions(input: string): string[]{
        return input.split(`

`);
    }

    createStackArrays(stackInput: string): Stack[]{
        let horizontalLineArr = stackInput.split('\n')
            .map(line => {
                let characterCounter = 0;
                return line
                    .split('')
                    .reduce((stackArr, currentCharacter) => {
                        if(characterCounter === 0 || characterCounter === 2){
                            characterCounter++;//Next character should be parsed
                            return stackArr;
                        }
                        else if(characterCounter === 3){
                            characterCounter = 0;//Reset for next check thing
                            return stackArr;
                        }
                        else{
                            characterCounter++;
                            stackArr.push((currentCharacter === ' ' ? null : new Crate(currentCharacter)));
                            return stackArr;
                        }
                    }, [] as (Crate | null)[]);
            });
        if(!horizontalLineArr.every(l => l.length === horizontalLineArr[0].length)){
            throw new Error('Cannot parse crate stacks, array length was not equal: ' + JSON.stringify(new Set(horizontalLineArr.map(l => l.length))));
        }
        let finalStackArr: Stack[] = [];
        horizontalLineArr.pop();//remove the numbers at the bottom
        for(let i = 0; i < horizontalLineArr[0].length; i++){
            finalStackArr.push(new Stack(horizontalLineArr.filter(l => l[i] !== null).map(l => l[i] as Crate).reverse()));
        }

        return finalStackArr;
    }

    getInstructions(instructionInput: string): Instruction[]{
        return instructionInput.split('\n').filter(l => l !== '').map(l => {
            const REPLACEMENT_CHARACTER = '_';
            let line = l.replace('move ', REPLACEMENT_CHARACTER)
                .replace(' from ', REPLACEMENT_CHARACTER)
                .replace(' to ', REPLACEMENT_CHARACTER)
                .split(REPLACEMENT_CHARACTER)
                .filter(c => c !== '');
            if(line.length !== 3){
                throw new Error(`Cannot parse instruction line. Length of instructions was: ${line.length}`);
            }
            return new Instruction(Number(line[0]), Number(line[1]), Number(line[2]));
        });
    }
}