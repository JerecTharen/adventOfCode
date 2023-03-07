import Parser from "./Parser";

export default class Crate{
    constructor(private _parser: Parser, private _isMover9001: boolean = false){
        this._isMover9001;
    }
    followInstructions():void{
        this._parser.instructions.forEach(instruction => {
            if(!this._isMover9001){
                for(let i = 0; i < instruction.MoveQauntity; i++){
                    const crate = this._parser.stacks[instruction.PickupLocation - 1].GrabCrate();
                    if(crate === undefined){
                        throw new Error(
                            `Cannot execute instruction: move ${instruction.MoveQauntity} from ${instruction.PickupLocation} to ${instruction.PlaceLocation} for crate ${i + 1}`
                        );
                    }
                    this._parser.stacks[instruction.PlaceLocation - 1].StackCrate(crate!);
                }
            }
            else{
                let heldCrates = [];
                for(let i = 0; i < instruction.MoveQauntity; i++){
                    const crate = this._parser.stacks[instruction.PickupLocation - 1].GrabCrate();
                    if(crate === undefined){
                        throw new Error(
                            `Cannot execute instruction: move ${instruction.MoveQauntity} from ${instruction.PickupLocation} to ${instruction.PlaceLocation} for crate ${i + 1}`
                        );
                    }
                    heldCrates.push(crate);
                }
                heldCrates.reverse().forEach(crate => {
                    this._parser.stacks[instruction.PlaceLocation - 1].StackCrate(crate);
                });
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