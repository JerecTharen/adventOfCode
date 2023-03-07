import Crate from '../lib/Crate';
import Stack from '../lib/Stack';
describe('stack', ()=>{
    it('should stack crate first in last out', ()=>{
        const EXPECTED_CHAR = 'z';

        let stubStack = new Stack([]);
        stubStack.StackCrate(new Crate(EXPECTED_CHAR));//Order matters
        stubStack.StackCrate(new Crate('a'));//Not expected character
        stubStack.GrabCrate();
        let actualChar = stubStack.GrabCrate()?.Char;

        expect(actualChar).toBe(EXPECTED_CHAR);
    });
})