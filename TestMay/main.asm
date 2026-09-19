global _start

section .text

_start:
  mov rdi, 11
  jmp exit

exit:
  mov rax, 60
  syscall
