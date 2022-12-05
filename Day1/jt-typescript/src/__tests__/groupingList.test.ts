import GroupingList from "../groupingList";

describe('groupingList', ()=>{
    const baseInput = `1
    2
    3
    
    4
    5
    6`;
    let groupingList: GroupingList;
    beforeEach(()=>{
        groupingList = new GroupingList(baseInput);
    })
    it('should be able to separate an input with 2 groups from the base input string', ()=>{
        const expectedLength = 2;
        groupingList.parseInput();
        const actualLength = groupingList.groupInputs.length;
        expect(actualLength).toBe(expectedLength);
    });
})