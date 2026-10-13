<?php

use PHPUnit\Framework\TestCase;
use App\Solution;

class SolutionTest extends TestCase {
    public function test(): void {
        $hello = new Solution();
        $actual = $hello->topKFrequent([1,1,1,2,2,3], 2);
        $this->assertEquals([1, 2], $actual);
    }

    public function testNegative(): void {
        $hello = new Solution();
        $actual = $hello->topKFrequent([-1,-1], 1);
        $this->assertEquals([-1], $actual);
    }

    public function testCase15(): void {
        $hello = new Solution();
        $actual = $hello->topKFrequent([5,3,1,1,1,3,73,1], 1);
        $this->assertEquals([1], $actual);
    }
}
