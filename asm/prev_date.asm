asm_prev_date:
	movabs rax, 281474976710655
	and rax, rdi
	mov rcx, rdi
	shr rcx, 32
	mov rdx, rdi
	shr rdx, 40
	cmp dl, 1
	jbe .LBB10_1
	dec dl
	jmp .LBB10_6
.LBB10_1:
	cmp cl, 1
	jbe .LBB10_2
	dec cl
	cmp cl, 2
	jne .LBB10_5
	imul ecx, edi, -1030792151
	add ecx, 85899345
	cmp ecx, 171798691
	mov ecx, 15
	mov edx, 3
	cmovb edx, ecx
	test edx, edi
	sete dl
	or dl, 28
	mov cl, 2
	jmp .LBB10_6
.LBB10_2:
	mov ecx, 4294967295
	add rax, rcx
	mov dl, 31
	mov cl, 12
	jmp .LBB10_6
.LBB10_5:
	mov edx, ecx
	shr dl, 3
	xor dl, cl
	or dl, 30
.LBB10_6:
	movzx edx, dl
	shl rdx, 40
	movzx ecx, cl
	shl rcx, 32
	or rcx, rdx
	mov eax, eax
	or rax, rcx
	ret
