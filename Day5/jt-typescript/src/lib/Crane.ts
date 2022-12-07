import Parser from "./Parser";

export default class Crate{
    constructor(private _parser: Parser){

    }
    followInstructions():void{
        this._parser.instructions.forEach(instruction => {
            for(let i = 0; i < instruction.MoveQauntity; i++){
                const crate = this._parser.stacks[instruction.PickupLocation - 1].GrabCrate();
                if(crate === undefined){
                    throw new Error(
                        `Cannot execute instruction: move ${instruction.MoveQauntity} from ${instruction.PickupLocation} to ${instruction.PlaceLocation} for crate ${i + 1}`
                    );
                }
                this._parser.stacks[instruction.PlaceLocation - 1].StackCrate(crate!);
            }
        });
    }
    getTopCrates():string{
        return this._parser.stacks
            .map(s => s.GrabCrate())
            .filter(c => c !== undefined)
            .map(c => c!.Char)
            .reduce((topCrates:string, currentCrate: string)=>{
                return topCrates += currentCrate!;
            }, '');
    }
}