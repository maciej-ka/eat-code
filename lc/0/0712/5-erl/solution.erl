%% cannot allocate 8048 bytes of memory (of type "db_term")

-module(solution).
-include_lib("eunit/include/eunit.hrl").

-spec minimum_delete_sum(S1 :: unicode:unicode_binary(), S2 :: unicode:unicode_binary()) -> integer().
minimum_delete_sum(S1, S2) ->
    memo_solve(
        binary_to_list(S1),
        binary_to_list(S2),
        ets:new(memo, [set, private])).

memo_solve(A, B, Memo) ->
    case ets:lookup(Memo, {A, B}) of
        [{_, Res}] -> Res;
        [] ->
            Res = solve(A, B, Memo),
            ets:insert(Memo, {{A, B}, Res}),
            Res
    end.

solve([], [], _) -> 0;
solve([A | AS], [], Memo) -> A + memo_solve(AS, [], Memo);
solve([], [B | BS], Memo) -> B + memo_solve([], BS, Memo);
solve([X | AS], [X | BS], Memo) -> memo_solve(AS, BS, Memo);
solve([A | AS], [B | BS], Memo) ->
    min(
        A + memo_solve(AS, [B | BS], Memo),
        B + memo_solve([A | AS], BS, Memo)).

solution_1_test() ->
    ?assertEqual(231, minimum_delete_sum(<<"sea">>, <<"eat">>)).

solution_2_test() ->
    ?assertEqual(403, minimum_delete_sum(<<"delete">>, <<"leet">>)).

solution_3_test() ->
    ?assertEqual(41731, minimum_delete_sum(<<"igijekdtywibepwonjbwykkqmrgmtybwhwjiqudxmnniskqjfbkpcxukrablqmwjndlhblxflgehddrvwfacarwkcpmcfqnajqfxyqwiugztocqzuikamtvmbjrypfqvzqiwooewpzcpwhdejmuahqtukistxgfafrymoaodtluaexucnndlnpeszdfsvfofdylcicrrevjggasrgdhwdgjwcchyanodmzmuqeupnpnsmdkcfszznklqjhjqaboikughrnxxggbfyjriuvdsusvmhiaszicfa">>, <<"ikhuivqorirphlzqgcruwirpewbjgrjtugwpnkbrdfufjsmgzzjespzdcdjcoioaqybciofdzbdieegetnogoibbwfielwungehetanktjqjrddkrnsxvdmehaeyrpzxrxkhlepdgpwhgpnaatkzbxbnopecfkxoekcdntjyrmmvppcxcgquhomcsltiqzqzmkloomvfayxhawlyqxnsbyskjtzxiyrsaobbnjpgzmetpqvscyycutdkpjpzfokvi">>)).

