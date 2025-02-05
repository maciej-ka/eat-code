const { topKFrequent } = require("./program");

describe('top k frequent', () => {
  it('works', () => {
    const actual = topKFrequent([1,1,1,2,2,3], 2);
    expect(actual).toEqual([1,2]);
  });
});
