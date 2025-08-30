; https://leetcode.com/problems/valid-sudoku/submissions/1753662192/?envType=daily-question&envId=2025-08-30
#lang racket
(provide is-valid-sudoku)

(define/contract (is-valid-sudoku board)
  (-> (listof (listof char?)) boolean?)
    (valid? board 0 0 (list
      (make-vector 90 0) ; rows
      (make-vector 90 0) ; cols
      (make-vector 90 0)))) ; areas

; board without first cell
(define (board-rest board)
  (append
    (list (rest (first board)))
    (rest board)))

; board first element as number
(define (board-first board)
  (string->number (string
    (first (first board)))))

(define (seen? num vec)
  (if (= (vector-ref vec num) 0)
    (vector-set! vec num 1)
    #f))

(define (checks-ok? num row col counts)
  (and
    (seen? (+ (* 10 row) num) (first counts))
    (seen? (+ (* 10 col) num) (second counts))
    (seen? (+ (* 30 (quotient col 3)) (* 10 (quotient row 3)) num) (third counts))))

(define (valid? board row col counts)
  (cond
    [(null? board) #t]
    [(null? (first board)) (valid? (rest board) (add1 row) 0 counts)]
    [else (let ([num (board-first board)])
      (if (or (not num) (checks-ok? num row col counts))
        (valid? (board-rest board) row (add1 col) counts)
        #f))]))
