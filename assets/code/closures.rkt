#lang racket

;; CIS352 — Closure-Creating Interpreters
;;
;; This file contains:
;;   1. The language definition (expr?)
;;   2. The standard interpreter (interp1)
;;   3. The certifying interpreter (interp-cert) — generates proof trees
;;   4. The serializer (prove) — outputs S-expressions for the proof tree viewer
;;
;; Run examples at the bottom, then paste the output into the
;; proof tree viewer at: https://kmicinski.com/cis352-s26/closures

;; ─── The Language ──────────────────────────────────────

(define (expr? e)
  (match e
    [(? symbol? x) #t]
    [`(,(? expr? e0) ,(? expr? e1)) #t]
    [`(λ (,(? symbol? x)) ,(? expr? e-body)) #t]
    [(? integer? i) #t]
    [`(- ,(? expr? e)) #t]
    [`(+ ,(? expr? e0) ,(? expr? e1)) #t]
    [`(let ([,(? symbol? x) ,(? expr? e)]) ,(? expr? e-b)) #t]
    [`(if0 ,(? expr? e-guard) ,(? expr? e-true) ,(? expr? e-false)) #t]
    [_ #f]))

;; ─── Values ────────────────────────────────────────────

(define (value? v)
  (match v
    [(? integer? i) #t]
    [`(clo (λ (,x) ,e) ,(? hash? env)) #t]
    [_ #f]))

;; ─── The Standard Interpreter ──────────────────────────

(define (interp1 env e)
  (match e
    [(? symbol? x) (hash-ref env x)]
    [(? integer? i) i]
    [`(- ,e) (- (interp1 env e))]
    [`(+ ,e0 ,e1)
     (let ([v0 (interp1 env e0)]
           [v1 (interp1 env e1)])
       (+ v0 v1))]
    [`(if0 ,e-guard ,e-true ,e-false)
     (let ([vg (interp1 env e-guard)])
       (if (= vg 0)
           (interp1 env e-true)
           (interp1 env e-false)))]
    [`(let ([,x ,e0]) ,e-body)
     (let ([v (interp1 env e0)])
       (interp1 (hash-set env x v) e-body))]
    [`(λ (,x) ,e+) `(clo (λ (,x) ,e+) ,env)]
    [`(,e0 ,e1)
     (match (interp1 env e0)
       [`(clo (λ (,x) ,e+) ,env+)
        (define new-env (hash-set env+ x (interp1 env e1)))
        (interp1 new-env e+)])]))

;; ─── Formatting Helpers ────────────────────────────────

(define (fmt-env env)
  (if (hash-empty? env) "{}"
      (format "{~a}"
        (string-join
          (for/list ([k (sort (hash-keys env) symbol<?)])
            (format "~a ↦ ~a" k (fmt-val (hash-ref env k))))
          ", "))))

(define (fmt-val v)
  (match v
    [(? integer?) (~a v)]
    [`(clo (λ (,x) ,e) ,env)
     (format "⟨λ (~a) ~a , ~a⟩" x e (fmt-env env))]))

(define (fmt-judge env e v)
  (format "~a ⊢ ~a ⇓ ~a" (fmt-env env) e (fmt-val v)))

;; ─── The Certifying Interpreter ────────────────────────
;;
;; Identical to interp1, but each clause returns (cons value proof-node).
;; Proof nodes have the shape: (RuleName premise ... --- conclusion)

(define (interp-cert env e)
  (match e
    [(? symbol? x)
     (define v (hash-ref env x))
     (cons v `(Var ,(format "~a(~a) = ~a" (fmt-env env) x (fmt-val v))
                   --- ,(fmt-judge env e v)))]
    [(? integer? i)
     (cons i `(Int --- ,(fmt-judge env e i)))]
    [`(- ,e0)
     (match-define (cons v0 pf0) (interp-cert env e0))
     (define v (- v0))
     (cons v `(Neg ,pf0 ,(format "v = ~a" v)
                   --- ,(fmt-judge env e v)))]
    [`(+ ,e0 ,e1)
     (match-define (cons v0 pf0) (interp-cert env e0))
     (match-define (cons v1 pf1) (interp-cert env e1))
     (define v (+ v0 v1))
     (cons v `(Add ,pf0 ,pf1 ,(format "v = ~a + ~a" v0 v1)
                   --- ,(fmt-judge env e v)))]
    [`(if0 ,e-guard ,e-true ,e-false)
     (match-define (cons vg pfg) (interp-cert env e-guard))
     (if (= vg 0)
         (match (interp-cert env e-true)
           [(cons v pf)
            (cons v `(If0-True ,pfg ,pf
                               --- ,(fmt-judge env e v)))])
         (match (interp-cert env e-false)
           [(cons v pf)
            (cons v `(If0-False ,pfg ,(format "~a ≠ 0" vg) ,pf
                                --- ,(fmt-judge env e v)))]))]
    [`(let ([,x ,e0]) ,e-body)
     (match-define (cons v0 pf0) (interp-cert env e0))
     (match-define (cons v pf-body)
       (interp-cert (hash-set env x v0) e-body))
     (cons v `(Let ,pf0 ,pf-body --- ,(fmt-judge env e v)))]
    [`(λ (,x) ,e+)
     (define v `(clo (λ (,x) ,e+) ,env))
     (cons v `(Lam --- ,(fmt-judge env e v)))]
    [`(,e0 ,e1)
     (match-define (cons v0 pf0) (interp-cert env e0))
     (match-define (cons v1 pf1) (interp-cert env e1))
     (match v0
       [`(clo (λ (,x) ,e+) ,env+)
        (match-define (cons v pf-body)
          (interp-cert (hash-set env+ x v1) e+))
        (cons v `(App ,pf0 ,pf1 ,pf-body
                      --- ,(fmt-judge env e v)))])]))

;; ─── Proof Serializer ──────────────────────────────────
;;
;; Converts nested Racket lists to the S-expression string
;; format consumed by the proof tree viewer.

(define (proof->sexp pf)
  (match pf
    [`(,name ,@rest)
     (format "(~a)"
       (string-join
         (cons (format "(~a :right)" name)
               (for/list ([x (in-list rest)])
                 (cond
                   [(equal? x '---) "---"]
                   [(string? x) (format "\"~a\"" x)]
                   [else (proof->sexp x)])))
         " "))]))

(define (prove e)
  (proof->sexp (cdr (interp-cert (hash) e))))

;; ─── Examples ──────────────────────────────────────────
;;
;; Run this file, then copy any output line and paste it into the
;; proof tree viewer on the lecture page.

(displayln "=== Standard interpreter ===")
(displayln (interp1 (hash) '(+ 3 5)))
(displayln (interp1 (hash) '((λ (x) (+ x 1)) 5)))
(displayln (interp1 (hash) '(let ([x 10]) ((λ (y) (+ x y)) 7))))
(newline)

(displayln "=== Proof trees (paste into viewer) ===")
(displayln (prove '(+ 3 5)))
(newline)
(displayln (prove '((λ (x) (+ x 1)) 5)))
(newline)
(displayln (prove '(let ([x 10]) ((λ (y) (+ x y)) 7))))
