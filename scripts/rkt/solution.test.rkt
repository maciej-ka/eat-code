#lang racket
(require rackunit rackunit/text-ui "solution.rkt")

(define tests
  (test-suite "solution suite"

    (test-case "test 1"
      (check-equal?
        (solve '(1 2 3))
        3
      ))

    ))
(run-tests tests)
