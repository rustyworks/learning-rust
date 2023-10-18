	.text
	.file	"main.8941938a4efc4c71-cgu.0"
	.section	.text._ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h92dc85d3d96ab7edE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h92dc85d3d96ab7edE,@function
_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h92dc85d3d96ab7edE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN4core3ops8function6FnOnce9call_once17h5319a9e034a2cc3eE
	#APP
	#NO_APP
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h92dc85d3d96ab7edE, .Lfunc_end0-_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h92dc85d3d96ab7edE
	.cfi_endproc

	.section	.text._ZN3std2rt10lang_start17h84547c9ed02507caE,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17h84547c9ed02507caE
	.globl	_ZN3std2rt10lang_start17h84547c9ed02507caE
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start17h84547c9ed02507caE,@function
_ZN3std2rt10lang_start17h84547c9ed02507caE:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movl	%ecx, %eax
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, 16(%rsp)
	leaq	16(%rsp), %rdi
	leaq	.L__unnamed_1(%rip), %rsi
	movzbl	%al, %r8d
	callq	*_ZN3std2rt19lang_start_internal17h2bbe0b58b2b89a9fE@GOTPCREL(%rip)
	movq	%rax, 8(%rsp)
	movq	8(%rsp), %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	_ZN3std2rt10lang_start17h84547c9ed02507caE, .Lfunc_end1-_ZN3std2rt10lang_start17h84547c9ed02507caE
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ddcf7e9fcc5ff5fE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ddcf7e9fcc5ff5fE,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ddcf7e9fcc5ff5fE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h92dc85d3d96ab7edE
	callq	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6dbbbb715c86aa26E
	movb	%al, 7(%rsp)
	movzbl	7(%rsp), %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ddcf7e9fcc5ff5fE, .Lfunc_end2-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ddcf7e9fcc5ff5fE
	.cfi_endproc

	.section	.text._ZN4core3fmt9Arguments9new_const17h133696362e8974edE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3fmt9Arguments9new_const17h133696362e8974edE,@function
_ZN4core3fmt9Arguments9new_const17h133696362e8974edE:
	.cfi_startproc
	subq	$104, %rsp
	.cfi_def_cfa_offset 112
	movq	%rdx, 8(%rsp)
	movq	%rsi, 16(%rsp)
	movq	%rdi, 24(%rsp)
	movq	%rdi, 32(%rsp)
	cmpq	$1, %rdx
	ja	.LBB3_2
	movq	32(%rsp), %rax
	movq	24(%rsp), %rcx
	movq	8(%rsp), %rdx
	movq	16(%rsp), %rsi
	movq	$0, 88(%rsp)
	movq	%rsi, (%rcx)
	movq	%rdx, 8(%rcx)
	movq	88(%rsp), %rsi
	movq	96(%rsp), %rdx
	movq	%rsi, 32(%rcx)
	movq	%rdx, 40(%rcx)
	leaq	.L__unnamed_2(%rip), %rdx
	movq	%rdx, 16(%rcx)
	movq	$0, 24(%rcx)
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB3_2:
	.cfi_def_cfa_offset 112
	leaq	.L__unnamed_3(%rip), %rsi
	leaq	40(%rsp), %rdi
	movq	%rdi, (%rsp)
	movl	$1, %edx
	callq	_ZN4core3fmt9Arguments9new_const17h133696362e8974edE
	movq	(%rsp), %rdi
	leaq	.L__unnamed_4(%rip), %rsi
	movq	_ZN4core9panicking9panic_fmt17h98ef273141454c23E@GOTPCREL(%rip), %rax
	callq	*%rax
	ud2
.Lfunc_end3:
	.size	_ZN4core3fmt9Arguments9new_const17h133696362e8974edE, .Lfunc_end3-_ZN4core3fmt9Arguments9new_const17h133696362e8974edE
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he515a32351fe7694E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he515a32351fe7694E,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he515a32351fe7694E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN4core3ops8function6FnOnce9call_once17h180f6f7077e05881E
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end4:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he515a32351fe7694E, .Lfunc_end4-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he515a32351fe7694E
	.cfi_endproc

	.section	.text._ZN4core3ops8function6FnOnce9call_once17h180f6f7077e05881E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce9call_once17h180f6f7077e05881E,@function
_ZN4core3ops8function6FnOnce9call_once17h180f6f7077e05881E:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
.Ltmp0:
	leaq	8(%rsp), %rdi
	callq	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ddcf7e9fcc5ff5fE
.Ltmp1:
	movl	%eax, 4(%rsp)
	jmp	.LBB5_3
.LBB5_1:
	movq	24(%rsp), %rdi
	callq	_Unwind_Resume@PLT
	ud2
.LBB5_2:
.Ltmp2:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 24(%rsp)
	movl	%eax, 32(%rsp)
	jmp	.LBB5_1
.LBB5_3:
	movl	4(%rsp), %eax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end5:
	.size	_ZN4core3ops8function6FnOnce9call_once17h180f6f7077e05881E, .Lfunc_end5-_ZN4core3ops8function6FnOnce9call_once17h180f6f7077e05881E
	.cfi_endproc
	.section	.gcc_except_table._ZN4core3ops8function6FnOnce9call_once17h180f6f7077e05881E,"a",@progbits
	.p2align	2, 0x0
GCC_except_table5:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.uleb128 .Lfunc_end5-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	.text._ZN4core3ops8function6FnOnce9call_once17h5319a9e034a2cc3eE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce9call_once17h5319a9e034a2cc3eE,@function
_ZN4core3ops8function6FnOnce9call_once17h5319a9e034a2cc3eE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	*%rdi
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end6:
	.size	_ZN4core3ops8function6FnOnce9call_once17h5319a9e034a2cc3eE, .Lfunc_end6-_ZN4core3ops8function6FnOnce9call_once17h5319a9e034a2cc3eE
	.cfi_endproc

	.section	".text._ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h725db8f9b2ab6d8eE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h725db8f9b2ab6d8eE,@function
_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h725db8f9b2ab6d8eE:
	.cfi_startproc
	retq
.Lfunc_end7:
	.size	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h725db8f9b2ab6d8eE, .Lfunc_end7-_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h725db8f9b2ab6d8eE
	.cfi_endproc

	.section	".text._ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6dbbbb715c86aa26E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6dbbbb715c86aa26E,@function
_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6dbbbb715c86aa26E:
	.cfi_startproc
	xorl	%eax, %eax
	retq
.Lfunc_end8:
	.size	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6dbbbb715c86aa26E, .Lfunc_end8-_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6dbbbb715c86aa26E
	.cfi_endproc

	.section	.text._ZN4main4main17hb9bf57f50a56af79E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4main4main17hb9bf57f50a56af79E,@function
_ZN4main4main17hb9bf57f50a56af79E:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	leaq	8(%rsp), %rdi
	leaq	.L__unnamed_5(%rip), %rsi
	movl	$1, %edx
	callq	_ZN4core3fmt9Arguments9new_const17h133696362e8974edE
	leaq	8(%rsp), %rdi
	callq	*_ZN3std2io5stdio6_print17hbaebff3de335da69E@GOTPCREL(%rip)
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end9:
	.size	_ZN4main4main17hb9bf57f50a56af79E, .Lfunc_end9-_ZN4main4main17hb9bf57f50a56af79E
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rdx
	movslq	%edi, %rsi
	leaq	_ZN4main4main17hb9bf57f50a56af79E(%rip), %rdi
	xorl	%ecx, %ecx
	callq	_ZN3std2rt10lang_start17h84547c9ed02507caE
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end10:
	.size	main, .Lfunc_end10-main
	.cfi_endproc

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_1:
	.quad	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h725db8f9b2ab6d8eE
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he515a32351fe7694E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ddcf7e9fcc5ff5fE
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ddcf7e9fcc5ff5fE
	.size	.L__unnamed_1, 48

	.type	.L__unnamed_2,@object
	.section	.rodata..L__unnamed_2,"a",@progbits
	.p2align	3, 0x0
.L__unnamed_2:
	.size	.L__unnamed_2, 0

	.type	.L__unnamed_6,@object
	.section	.rodata..L__unnamed_6,"a",@progbits
.L__unnamed_6:
	.ascii	"invalid args"
	.size	.L__unnamed_6, 12

	.type	.L__unnamed_3,@object
	.section	.data.rel.ro..L__unnamed_3,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_3:
	.quad	.L__unnamed_6
	.asciz	"\f\000\000\000\000\000\000"
	.size	.L__unnamed_3, 16

	.type	.L__unnamed_7,@object
	.section	.rodata..L__unnamed_7,"a",@progbits
.L__unnamed_7:
	.ascii	"/rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3/library/core/src/fmt/mod.rs"
	.size	.L__unnamed_7, 75

	.type	.L__unnamed_4,@object
	.section	.data.rel.ro..L__unnamed_4,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_4:
	.quad	.L__unnamed_7
	.asciz	"K\000\000\000\000\000\000\000+\001\000\000\r\000\000"
	.size	.L__unnamed_4, 24

	.type	.L__unnamed_8,@object
	.section	.rodata..L__unnamed_8,"a",@progbits
.L__unnamed_8:
	.ascii	"Hello, world!\n"
	.size	.L__unnamed_8, 14

	.type	.L__unnamed_5,@object
	.section	.data.rel.ro..L__unnamed_5,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_5:
	.quad	.L__unnamed_8
	.asciz	"\016\000\000\000\000\000\000"
	.size	.L__unnamed_5, 16

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"aGw",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.section	".note.GNU-stack","",@progbits
