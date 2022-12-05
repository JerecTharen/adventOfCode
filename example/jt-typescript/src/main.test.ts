import {main} from './main';

describe('Puzzle Solution Examples', ()=>{
  it('should solve the first example to 5 blocks', ()=>{
    const input = "R2, L3";
    const blockCount = main(input);
    expect(blockCount).toBe(5);
  });
  it('should solve the second example to 2 blocks', ()=>{
    const input = "R2, R2, R2";
    const blockCount = main(input);
    expect(blockCount).toBe(2);
  });
  it('should solve the third example to 12 blocks', ()=>{
    const input = "R5, L5, R5, R3";
    const blockCount = main(input);
    expect(blockCount).toBe(12);
  });
})
