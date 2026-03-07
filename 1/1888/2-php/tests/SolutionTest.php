<?php
use PHPUnit\Framework\TestCase;
use App\Solution;

class SolutionTest extends TestCase {

    public function test1(): void {
        $hello = new Solution();
        $actual = $hello->minFlips("111000");
        $expected = 2;
        $this->assertEquals($expected, $actual);
    }

    public function test2(): void {
        $hello = new Solution();
        $actual = $hello->minFlips("010");
        $expected = 0;
        $this->assertEquals($expected, $actual);
    }

    public function test3(): void {
        $hello = new Solution();
        $actual = $hello->minFlips("1110");
        $expected = 1;
        $this->assertEquals($expected, $actual);
    }

    public function test4(): void {
        $hello = new Solution();
        $actual = $hello->minFlips("101001010");
        $expected = 0;
        $this->assertEquals($expected, $actual);
    }

    public function test5(): void {
        $hello = new Solution();
        $actual = $hello->minFlips("101100101");
        $expected = 2;
        $this->assertEquals($expected, $actual);
    }

}
