asm_days_in_month:
	cmp sil, 2
	jne .LBB16_2
	imul eax, edi, -1030792151
	add eax, 85899345
	cmp eax, 171798691
	mov eax, 15
	mov ecx, 3
	cmovb ecx, eax
	test ecx, edi
	sete al
	or al, 28
	ret
.LBB16_2:
	mov eax, esi
	shr al, 3
	xor al, sil
	or al, 30
	ret
