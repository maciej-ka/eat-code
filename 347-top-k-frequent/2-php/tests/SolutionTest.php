<?php

use PHPUnit\Framework\TestCase;
use App\Solution;

class SolutionTest extends TestCase {
    public function testSayHello() {
        $hello = new Solution();
        $this->assertEquals("Hello, World!", $hello->sayHello());
    }
}
