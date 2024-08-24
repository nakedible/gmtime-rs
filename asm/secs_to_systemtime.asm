asm_secs_to_systemtime:
	mov eax, esi
	mov rsi, rdi
	test rdi, rdi
	js .LBB23_1
	cmp eax, 1000000000
	jb .LBB23_5
	mov ecx, eax
	shr ecx, 9
	imul rcx, rcx, 281475
	shr rcx, 39
	add rsi, rcx
	imul ecx, ecx, 1000000000
	sub eax, ecx
.LBB23_5:
	lea rdi, [rip + .L__unnamed_2]
	mov edx, eax
	jmp qword ptr [rip + std::time::SystemTime::checked_add@GOTPCREL]
.LBB23_1:
	test eax, eax
	je .LBB23_2
	not rsi
	mov edx, 1000000000
	sub edx, eax
	cmp eax, 1000000001
	jb .LBB23_8
	mov eax, edx
	shr eax, 9
	imul rax, rax, 281475
	shr rax, 39
	add rsi, rax
	imul eax, eax, 1000000000
	sub edx, eax
.LBB23_8:
	lea rdi, [rip + .L__unnamed_2]
	jmp qword ptr [rip + std::time::SystemTime::checked_sub@GOTPCREL]
.LBB23_2:
	neg rsi
	lea rdi, [rip + .L__unnamed_2]
	xor edx, edx
	jmp qword ptr [rip + std::time::SystemTime::checked_sub@GOTPCREL]
