#lang racket
(provide sort-matrix)

(define/contract (sort-matrix grid)
  (-> (listof (listof exact-integer?)) (listof (listof exact-integer?)))
    grid
  )

