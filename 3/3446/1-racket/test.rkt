#lang racket
(require rackunit rackunit/text-ui "solution.rkt")

(define tests
  (test-suite "solution tests"

    (test-case "test 1"
      (check-equal?
        (sort-matrix '((1 7 3) (9 8 2) (4 5 6)))
        '((8 2 3) (9 6 7) (4 5 1))
      ))

    (test-case "test 2"
      (check-equal?
        (sort-matrix '((0 1) (1 2)))
        '((2 1) (1 0))
      ))

    (test-case "test 3"
      (check-equal?
        (sort-matrix '((1)))
        '((1))
      ))

    ))
(run-tests tests)
