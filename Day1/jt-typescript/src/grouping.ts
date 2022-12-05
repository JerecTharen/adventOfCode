export default class Grouping{
    private _lines: string[] = [];
    private _groupCalories: number[] = [];
    constructor(private _unparsedGroup: string) {}

    public parse(): void{
        this._lines = this._unparsedGroup.split(/\r?\n/).map(l => l.trim());//Have to trim off some extra spacing
        if(this._lines.length === 0){
            throw new Error('Cannot parse input. Must be separated by newlines');
        }
        this._groupCalories = this._lines.map(l => {
            if(Number.isNaN(l)){
                throw new Error(`Cannot parse: ${l}, all items must be numbers`);
            }
            return Number(l);
        });
    }
    public get Lines(){
        if(this._lines.length === 0){
            throw new Error('Parse must be run first');
        }
        return this._lines;
    }
    public get Sum(): number{
        if(this._groupCalories.length === 0){
            throw new Error('Parse must be run first');
        }
        return this._groupCalories.reduce((sum, calorie) => sum + calorie, 0);
    }
}