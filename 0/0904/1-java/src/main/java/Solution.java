// https://leetcode.com/problems/fruit-into-baskets/submissions/1723091727/?envType=daily-question&envId=2025-08-04

class Solution {
    public int totalFruit(int[] fruits) {
        if (fruits.length < 2) {
            return fruits.length;
        }

        int result = 0;
        int start = 0;
        // two baskets
        int fruita = -1;
        int fruitb = -1;
        // last occurence
        int ia = -1;
        int ib = -1;

        for (int i = 0; i < fruits.length; i++) {
            var fruit = fruits[i];
            // duplicate of most recent one
            if (fruit == fruita) {
                ia = i;
                continue;
            }
            // second most recent one
            if (fruit == fruitb) {
                // swap fruits
                int tmp = fruita;
                fruita = fruitb;
                fruitb = tmp;
                // swap last occurence
                tmp = ia;
                ia = ib;
                ib = tmp;
                // update last occurence
                ia = i;
                continue;
            }
            // new one
            result = Math.max(result, i - start);
            start = ib + 1;
            // add new
            fruitb = fruita;
            fruita = fruit;
            ib = ia;
            ia = i;
        }

        // last score
        result = Math.max(result, fruits.length - start);
        return result;
    }
}
