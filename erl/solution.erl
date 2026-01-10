-module(solution).
-include_lib("eunit/include/eunit.hrl").

-spec solve(Nums :: [integer()]) -> integer().
solve(Nums) ->
    length(Nums).

solve_1_test() ->
    ?assertEqual(3, solve([1, 2, 3])).
