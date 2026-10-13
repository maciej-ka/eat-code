%% https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/submissions/1883098558/?envType=daily-question&envId=2026-01-12

-module(solution).
-include_lib("eunit/include/eunit.hrl").

-spec minimum_delete_sum(S1 :: unicode:unicode_binary(), S2 :: unicode:unicode_binary()) -> integer().
minimum_delete_sum(S1, S2) ->
    erlang:erase(),
    memo_solve(binary_to_list(S1), binary_to_list(S2)).

memo_solve(A, B) ->
    case get({A, B}) of
        undefined ->
            Res = solve(A, B),
            put({A, B}, Res),
            Res;
        Res -> Res
    end.

solve([], []) -> 0;
solve([A | AS], []) -> A + memo_solve(AS, []);
solve([], [B | BS]) -> B + memo_solve([], BS);
solve([X | AS], [X | BS]) -> memo_solve(AS, BS);
solve([A | AS], [B | BS]) ->
    min(
        A + memo_solve(AS, [B | BS]),
        B + memo_solve([A | AS], BS)).

solution_1_test() ->
    ?assertEqual(231, minimum_delete_sum(<<"sea">>, <<"eat">>)).

solution_2_test() ->
    ?assertEqual(403, minimum_delete_sum(<<"delete">>, <<"leet">>)).

solution_3_test() ->
    ?assertEqual(41731, minimum_delete_sum(<<"igijekdtywibepwonjbwykkqmrgmtybwhwjiqudxmnniskqjfbkpcxukrablqmwjndlhblxflgehddrvwfacarwkcpmcfqnajqfxyqwiugztocqzuikamtvmbjrypfqvzqiwooewpzcpwhdejmuahqtukistxgfafrymoaodtluaexucnndlnpeszdfsvfofdylcicrrevjggasrgdhwdgjwcchyanodmzmuqeupnpnsmdkcfszznklqjhjqaboikughrnxxggbfyjriuvdsusvmhiaszicfa">>, <<"ikhuivqorirphlzqgcruwirpewbjgrjtugwpnkbrdfufjsmgzzjespzdcdjcoioaqybciofdzbdieegetnogoibbwfielwungehetanktjqjrddkrnsxvdmehaeyrpzxrxkhlepdgpwhgpnaatkzbxbnopecfkxoekcdntjyrmmvppcxcgquhomcsltiqzqzmkloomvfayxhawlyqxnsbyskjtzxiyrsaobbnjpgzmetpqvscyycutdkpjpzfokvi">>)).


