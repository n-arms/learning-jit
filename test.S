	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 13, 0	sdk_version 13, 3
	.globl	_main                           ## -- Begin function main
	.p2align	4, 0x90
_main:                                  ## @main
	addq %rax, %rdi
	addq %rax, %rsi
	retq
                                        ## -- End function
.subsections_via_symbols
