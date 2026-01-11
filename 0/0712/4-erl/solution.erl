%% https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/submissions/1881926285/?envType=daily-question&envId=2026-01-10

-module(solution).
-include_lib("eunit/include/eunit.hrl").

-spec minimum_delete_sum(S1 :: unicode:unicode_binary(), S2 :: unicode:unicode_binary()) -> integer().
minimum_delete_sum(S1, S2) ->
    {Res, _} = memo_solve(binary_to_list(S1), binary_to_list(S2), #{}),
    Res.

memo_solve(A, B, Memo) ->
    case Memo of
        #{{A, B} := Res} -> {Res, Memo};
        _ ->
            {Res, Memo1} = solve(A, B, Memo),
            {Res, Memo1#{{A, B} => Res}}
    end.

solve([], [], Memo) -> {0, Memo};
solve([A | AS], [], Memo) ->
    {Res, Memo1} = memo_solve(AS, [], Memo),
    {A + Res, Memo1};
solve([], [B | BS], Memo) ->
    {Res, Memo1} = memo_solve([], BS, Memo),
    {B + Res, Memo1};
solve([X | AS], [X | BS], Memo) ->
    {Res, Memo1} = memo_solve(AS, BS, Memo),
    {Res, Memo1};
solve([A | AS], [B | BS], Memo) ->
    {ResA, MemoA} = memo_solve(AS, [B | BS], Memo),
    {ResB, MemoB} = memo_solve([A | AS], BS, MemoA),
    {min(A + ResA, B + ResB), MemoB}.

solution_1_test() ->
    ?assertEqual(231, minimum_delete_sum(<<"sea">>, <<"eat">>)).

solution_2_test() ->
    ?assertEqual(403, minimum_delete_sum(<<"delete">>, <<"leet">>)).

solution_3_test() ->
    ?assertEqual(41731, minimum_delete_sum(<<"igijekdtywibepwonjbwykkqmrgmtybwhwjiqudxmnniskqjfbkpcxukrablqmwjndlhblxflgehddrvwfacarwkcpmcfqnajqfxyqwiugztocqzuikamtvmbjrypfqvzqiwooewpzcpwhdejmuahqtukistxgfafrymoaodtluaexucnndlnpeszdfsvfofdylcicrrevjggasrgdhwdgjwcchyanodmzmuqeupnpnsmdkcfszznklqjhjqaboikughrnxxggbfyjriuvdsusvmhiaszicfa">>, <<"ikhuivqorirphlzqgcruwirpewbjgrjtugwpnkbrdfufjsmgzzjespzdcdjcoioaqybciofdzbdieegetnogoibbwfielwungehetanktjqjrddkrnsxvdmehaeyrpzxrxkhlepdgpwhgpnaatkzbxbnopecfkxoekcdntjyrmmvppcxcgquhomcsltiqzqzmkloomvfayxhawlyqxnsbyskjtzxiyrsaobbnjpgzmetpqvscyycutdkpjpzfokvi">>)).

