datealgo::asm::date_to_weekday:
	mov rax, rdi
	shr rax, 32
	movsx ecx, al
	lea edx, [rcx + 12]
	xor esi, esi
	cmp al, 3
	setl sil
	cmovge edx, ecx
	mov eax, edi
	sub eax, esi
	add eax, 1468000
	imul rcx, rax, 1374389535
	mov rsi, rcx
	shr rsi, 37
	shr rdi, 16
	sar edi, 24
	lea eax, [rax + 4*rax]
	shr eax, 2
	shr rcx, 39
	imul edx, edx, 979
	add edx, -2855
	shr edx, 5
	sub edi, esi
	add ecx, eax
	add ecx, edi
	add ecx, edx
	imul eax, ecx, 613566756
	shr eax, 29
	ret
