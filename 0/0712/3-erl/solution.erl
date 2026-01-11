%% too slow, needs memo
-module(solution).
-include_lib("eunit/include/eunit.hrl").

-spec minimum_delete_sum(S1 :: unicode:unicode_binary(), S2 :: unicode:unicode_binary()) -> integer().
minimum_delete_sum(S1, S2) ->
    solve(binary_to_list(S1), binary_to_list(S2)).

solve([], []) -> 0;
solve([A | AS], []) -> A + solve(AS, []);
solve([], [B | BS]) -> B + solve([], BS);
solve([X | AS], [X | BS]) -> solve(AS, BS);
solve([A | AS], [B | BS]) ->
    min(
        A + solve(AS, [B | BS]),
        B + solve([A | AS], BS)).

solution_1_test() ->
    ?assertEqual(231, minimum_delete_sum(<<"sea">>, <<"eat">>)).

solution_2_test() ->
    ?assertEqual(403, minimum_delete_sum(<<"delete">>, <<"leet">>)).

%% solution_3_test() ->
%%     ?assertEqual(0, minimum_delete_sum(<<"igijekdtywibepwonjbwykkqmrgmtybwhwjiqudxmnniskqjfbkpcxukrablqmwjndlhblxflgehddrvwfacarwkcpmcfqnajqfxyqwiugztocqzuikamtvmbjrypfqvzqiwooewpzcpwhdejmuahqtukistxgfafrymoaodtluaexucnndlnpeszdfsvfofdylcicrrevjggasrgdhwdgjwcchyanodmzmuqeupnpnsmdkcfszznklqjhjqaboikughrnxxggbfyjriuvdsusvmhiaszicfa">>, <<"ikhuivqorirphlzqgcruwirpewbjgrjtugwpnkbrdfufjsmgzzjespzdcdjcoioaqybciofdzbdieegetnogoibbwfielwungehetanktjqjrddkrnsxvdmehaeyrpzxrxkhlepdgpwhgpnaatkzbxbnopecfkxoekcdntjyrmmvppcxcgquhomcsltiqzqzmkloomvfayxhawlyqxnsbyskjtzxiyrsaobbnjpgzmetpqvscyycutdkpjpzfokvi">>)).

