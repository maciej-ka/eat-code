import { expect, test } from 'vitest'
import solution from "./solution";

test('case 1', () => {
  const actual = solution(["bread"], [["yeast","flour"]], ["yeast","flour","corn"]);
  expect(actual).toEqual(["bread"]);
});

test('case 2', () => {
  const actual = solution(["bread","sandwich"], [["yeast","flour"],["bread","meat"]], ["yeast","flour","meat"]);
  expect(actual).toEqual(["bread","sandwich"]);
});

test('case 3', () => {
  const actual = solution(["bread","sandwich","burger"], [["yeast","flour"],["bread","meat"],["sandwich","meat","bread"]], ["yeast","flour","meat"]);
  expect(actual).toEqual(["bread","sandwich","burger"]);
});

test('case 4: two cyclic', () => {
  const actual = solution(["bread", "magic"], [["flour","magic"], ["corn","bread"]], ["flour","corn"]);
  expect(actual).toEqual([]);
});

test('case 5: self recursion', () => {
  const actual = solution(["magic"], [["magic"]], ["flour"]);
  expect(actual).toEqual([]);
});

test('case 6', () => {
  const actual = solution(
    ["xevvq","izcad","p","we","bxgnm","vpio","i","hjvu","igi","anp","tokfq","z","kwdmb","g","qb","q","b","hthy"],
    [["wbjr"],["otr","fzr","g"],["fzr","wi","otr","xgp","wbjr","igi","b"],["fzr","xgp","wi","otr","tokfq","izcad","igi","xevvq","i","anp"],["wi","xgp","wbjr"],["wbjr","bxgnm","i","b","hjvu","izcad","igi","z","g"],["xgp","otr","wbjr"],["wbjr","otr"],["wbjr","otr","fzr","wi","xgp","hjvu","tokfq","z","kwdmb"],["xgp","wi","wbjr","bxgnm","izcad","p","xevvq"],["bxgnm"],["wi","fzr","otr","wbjr"],["wbjr","wi","fzr","xgp","otr","g","b","p"],["otr","fzr","xgp","wbjr"],["xgp","wbjr","q","vpio","tokfq","we"],["wbjr","wi","xgp","we"],["wbjr"],["wi"]],
    ["wi","otr","wbjr","fzr","xgp"]
  );
  expect(actual).toEqual(["xevvq","g","izcad","hjvu","bxgnm","tokfq","z","b","i","hthy"]);
});

test('case 7', () => {
  const actual = solution(
    ["ju","fzjnm","x","e","zpmcz","h","q"],
    [["d"],["hveml","f","cpivl"],["cpivl","zpmcz","h","e","fzjnm","ju"],["cpivl","hveml","zpmcz","ju","h"],["h","fzjnm","e","q","x"],["d","hveml","cpivl","q","zpmcz","ju","e","x"],["f","hveml","cpivl"]],
    ["f","hveml","cpivl","d"]
  );
  expect(actual).toEqual(["ju", "fzjnm", "q"]);
});

