<?php
use PHPUnit\Framework\TestCase;
use App\Solution;

class SolutionTest extends TestCase {

    public function test1(): void {
        $hello = new Solution();
        $actual = $hello->solve([1, 2, 3]);
        $expected = 3;
        $this->assertEquals($expected, $actual);
    }

}
