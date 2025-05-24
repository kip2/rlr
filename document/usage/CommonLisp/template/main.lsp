(defun read-line-split-by-space (line)
  (let ((result '()))
    (with-input-from-string (in line)
      (loop for num = (read in nil)
            while num
            do (push num result)))
    (nreverse result)))

(defun read-line-as-string ()
  (read-line))

; Workd with both strings and numbers.
(defun println-num (num)
  (format t "~a~%" num))

; Print list
; example:
; (println-lst '(1 2 3))
; > 1 2 3
(defun println-list (lst)
  (format t "~{~a~^ ~}~%" lst))

(defun solve ()
  ; todo: implemented me!
  nil)

(defun main ()
  (let* ((line (read-line-split-by-space (read-line))))
    (solve)))

; input sample code
; (defun main ()
;   (let ((input (read-line-as-string (read-line))))
;     (format t "input: ~a~%" input)))

(main)
