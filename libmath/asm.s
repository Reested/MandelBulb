	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 1	sdk_version 11, 1
	.globl	_asm_sqrt               
	.p2align	4, 0x90
_asm_sqrt:                              
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movss	%xmm0, -4(%rbp)
	movss	-4(%rbp), %xmm0         
	sqrtss	%xmm0, %xmm0
	popq	%rbp
	retq
	.cfi_endproc
                                        
	.globl	_asm_atan2              
	.p2align	4, 0x90
_asm_atan2:                             
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movss	%xmm0, -4(%rbp)
	movss	%xmm1, -8(%rbp)
	movss	-4(%rbp), %xmm0         
	movss	-8(%rbp), %xmm1         
	callq	_atan2f
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc
                                        
	.globl	_asm_pow                
	.p2align	4, 0x90
_asm_pow:                               
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movss	%xmm0, -4(%rbp)
	movss	%xmm1, -8(%rbp)
	movss	-4(%rbp), %xmm0         
	movss	-8(%rbp), %xmm1         
	callq	_powf
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc
                                        
	.globl	_asm_sin                
	.p2align	4, 0x90
_asm_sin:                               
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movss	%xmm0, -4(%rbp)
	movss	-4(%rbp), %xmm0         
	cvtss2sd	%xmm0, %xmm0
	callq	_sin
	cvtsd2ss	%xmm0, %xmm0
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc
                                        
	.globl	_asm_cos                
	.p2align	4, 0x90
_asm_cos:                               
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movss	%xmm0, -4(%rbp)
	movss	-4(%rbp), %xmm0         
	cvtss2sd	%xmm0, %xmm0
	callq	_cos
	cvtsd2ss	%xmm0, %xmm0
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc
                                        
	.globl	_asm_remap              
	.p2align	4, 0x90
_asm_remap:                             
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movss	%xmm0, -4(%rbp)
	movss	%xmm1, -8(%rbp)
	movss	%xmm2, -12(%rbp)
	movss	%xmm3, -16(%rbp)
	movss	%xmm4, -20(%rbp)
	movss	-16(%rbp), %xmm0        
	movss	-4(%rbp), %xmm1         
	subss	-8(%rbp), %xmm1
	movss	-20(%rbp), %xmm2        
	subss	-16(%rbp), %xmm2
	mulss	%xmm2, %xmm1
	movss	-12(%rbp), %xmm2        
	subss	-8(%rbp), %xmm2
	divss	%xmm2, %xmm1
	addss	%xmm1, %xmm0
	popq	%rbp
	retq
	.cfi_endproc
                                        
.subsections_via_symbols
