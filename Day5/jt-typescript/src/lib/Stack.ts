import Crate from "./Crate";

export default class Stack{
    private _stack: Crate[] = [];
    constructor(crates: Crate[]){
        this._stack = crates;
    }
    public GrabCrate():Crate | undefined{
        return this._stack.pop();
    }
    public StackCrate(crate: Crate):void{
        this._stack.push(crate);
    }
}