#lang racket

(define (calculate-slope x0 y0 x1 y1)
  (/ (- y1 y0) (- x1 x0)))

;; defining a value
(define pi 3.14)

;; defining a function
(define (plus-one x) (add1 x))

;; f(x,...) in math in racket is...
;; (define (f x ...) ...) where the last ... is the body
;; in functional programming, all functions evaluate to their bodies

;; BE CAREFUL, this will not run because 12 is not a function:
;; (+ 1 ((* 3 4)))


(define (f x)
  (if (> x 0)
      (* x 2)
      (* x -2)))

(define (collatz x)
  (cond [(= 0 (modulo x 2))  (/ x 2)]
        [else (+ 1 (* 3 x))]))

;; you could always rewrite a cond to use multiple ifs
#;(define (f x)
  (cond [(g x) 5]
        [(k x) 3]
        [else 6]))

#;(define (f x)
  (if (g x)
      5
      (if (k x)
          3
          6)))

(define (g x)
  
  (+ x 2)
  (displayln "Hello, world!"))


(define x 42)

(define (foo x) x)

(let ([x 2])
  (+ x 3))
