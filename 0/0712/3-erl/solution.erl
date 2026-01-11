%% too slow, needs memo
-module(solution).
-include_lib("eunit/include/eunit.hrl").

-spec minimum_delete_sum(S1 :: unicode:unicode_binary(), S2 :: unicode:unicode_binary()) -> integer().
minimum_delete_sum(S1, S2) ->
    solve(binary_to_list(S1), binary_to_list(S2)).

solve([], []) -> 0;
solve([H | R], []) -> H + solve(R, []);
solve([], [H | R]) -> H + solve([], R);
solve([H | R1], [H | R2]) -> solve(R1, R2);
solve([H1 | R1], [H2 | R2]) ->
    min(
        H1 + solve(R1, [H2 | R2]),
        H2 + solve([H1 | R1], R2)).

solution_1_test() ->
    ?assertEqual(231, minimum_delete_sum(<<"sea">>, <<"eat">>)).

solution_2_test() ->
    ?assertEqual(403, minimum_delete_sum(<<"delete">>, <<"leet">>)).

%% solution_3_test() ->
%%     ?assertEqual(0, minimum_delete_sum(<<"igijekdtywibepwonjbwykkqmrgmtybwhwjiqudxmnniskqjfbkpcxukrablqmwjndlhblxflgehddrvwfacarwkcpmcfqnajqfxyqwiugztocqzuikamtvmbjrypfqvzqiwooewpzcpwhdejmuahqtukistxgfafrymoaodtluaexucnndlnpeszdfsvfofdylcicrrevjggasrgdhwdgjwcchyanodmzmuqeupnpnsmdkcfszznklqjhjqaboikughrnxxggbfyjriuvdsusvmhiaszicfa">>, <<"ikhuivqorirphlzqgcruwirpewbjgrjtugwpnkbrdfufjsmgzzjespzdcdjcoioaqybciofdzbdieegetnogoibbwfielwungehetanktjqjrddkrnsxvdmehaeyrpzxrxkhlepdgpwhgpnaatkzbxbnopecfkxoekcdntjyrmmvppcxcgquhomcsltiqzqzmkloomvfayxhawlyqxnsbyskjtzxiyrsaobbnjpgzmetpqvscyycutdkpjpzfokvi">>)).

