asm_next_date:
	movabs rcx, 1099511627776
	movabs rax, 281474976710655
	and rax, rdi
	mov rdx, rax
	shr rdx, 32
	mov rsi, rdi
	shr rsi, 40
	cmp sil, 28
	jae .LBB9_1
.LBB9_3:
	add rdi, rcx
	movabs rcx, 280375465082880
	and rcx, rdi
.LBB9_8:
	movzx edx, dl
	shl rdx, 32
	mov eax, eax
	or rax, rcx
	or rax, rdx
	ret
.LBB9_1:
	cmp dl, 2
	jne .LBB9_2
	imul r8d, edi, -1030792151
	add r8d, 85899345
	cmp r8d, 171798691
	mov r8d, 15
	mov r9d, 3
	cmovb r9d, r8d
	test r9d, edi
	sete r8b
	or r8b, 28
	cmp r8b, sil
	ja .LBB9_3
	jmp .LBB9_7
.LBB9_2:
	mov r8d, edx
	shr r8b, 3
	xor r8b, dl
	or r8b, 30
	cmp r8b, sil
	ja .LBB9_3
	cmp dl, 12
	jae .LBB9_5
.LBB9_7:
	inc rdx
	jmp .LBB9_8
.LBB9_5:
	inc rax
	mov edx, 1
	jmp .LBB9_8
