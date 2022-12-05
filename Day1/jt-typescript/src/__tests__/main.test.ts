import { main } from '../main'

describe('Day 1 Test', ()=>{
  const input = `1000
  2000
  3000
  
  4000
  
  5000
  6000
  
  7000
  8000
  9000
  
  10000`;
  it('should be able to match the first example', ()=>{
    const expected = 24000;
    const actual = main(input);
    expect(actual).toBe(expected);
  });
  it('should be able to sum the top 3 calorie totals', ()=>{
    const expected = 45000;
    const actual = main(input, 3);
    expect(actual).toBe(expected);
  })
})
