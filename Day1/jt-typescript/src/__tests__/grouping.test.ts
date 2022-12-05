import Grouping from "../grouping";

describe('grouping', ()=>{
    let grouping: Grouping;
    const groupingInput = `1
    2
    3`;
    beforeEach(()=>{
        grouping = new Grouping(groupingInput);
    })
    it('should be able to separate lines from a group string', ()=>{
        const expected = ['1', '2', '3'];

        grouping.parse();
        const actual = grouping.Lines;

        expect(actual).toEqual(expected);
    });
    it('should be able to sum lines from the parsed list of calories', ()=>{
        const expected = 6;

        grouping.parse();
        const actual = grouping.Sum;

        expect(actual).toBe(expected);
    });
});