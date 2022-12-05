export default class{
    private _groups: string[] = [];
    constructor(private _unparsedInput: string) {}
    public parseInput():void{
        this._groups = this._unparsedInput.split(/\n\s*\n/);//Split on blank lines
        if(this._groups.length === 1){
            this._groups = [];//Reset groups
            throw new Error('Group List cannot be parsed');
        }
    }
    public get groupInputs(): string[]{
        if(this._groups.length === 0){
            throw new Error('Group List must be parsed first!');
        }
        return this._groups;
    }
}