const get_one = require('./index');

describe('index', ()=>{
    describe('get_one', ()=>{
        it('should return 1', ()=>{
            const one = get_one();
            expect(one).toBe(1);
        })
    })
})