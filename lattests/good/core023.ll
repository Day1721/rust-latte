
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        

define i32 @main () {
LABEL_0:
%IDENT_0_a = alloca i32
store i32 1, i32* %IDENT_0_a
%IDENT_0_b = alloca i32
store i32 2, i32* %IDENT_0_b
%IDENT_0_c = alloca i32
store i32 1, i32* %IDENT_0_c
%IDENT_0_d = alloca i32
store i32 2, i32* %IDENT_0_d
%IDENT_0_e = alloca i32
store i32 1, i32* %IDENT_0_e
%IDENT_0_f = alloca i32
store i32 2, i32* %IDENT_0_f
%IDENT_0_g = alloca i32
store i32 1, i32* %IDENT_0_g
%IDENT_0_h = alloca i32
store i32 2, i32* %IDENT_0_h
%IDENT_0_i = alloca i32
store i32 1, i32* %IDENT_0_i
%IDENT_0_j = alloca i32
store i32 2, i32* %IDENT_0_j
%IDENT_0_k = alloca i32
store i32 1, i32* %IDENT_0_k
%IDENT_0_l = alloca i32
store i32 2, i32* %IDENT_0_l
%IDENT_0_m = alloca i32
store i32 1, i32* %IDENT_0_m
%IDENT_0_n = alloca i32
store i32 2, i32* %IDENT_0_n
%r0 = load i32, i32* %IDENT_0_a
%r1 = load i32, i32* %IDENT_0_b
%r2 = load i32, i32* %IDENT_0_c
%r3 = load i32, i32* %IDENT_0_d
%r4 = load i32, i32* %IDENT_0_e
%r5 = load i32, i32* %IDENT_0_f
%r6 = load i32, i32* %IDENT_0_g
%r7 = load i32, i32* %IDENT_0_h
%r8 = load i32, i32* %IDENT_0_i
%r9 = load i32, i32* %IDENT_0_j
%r10 = load i32, i32* %IDENT_0_k
%r11 = load i32, i32* %IDENT_0_l
%r12 = load i32, i32* %IDENT_0_m
%r13 = load i32, i32* %IDENT_0_n
%r14 = call i32 @foo..I.I.I.I.I.I.I.I.I.I.I.I.I.I(i32 %r0, i32 %r1, i32 %r2, i32 %r3, i32 %r4, i32 %r5, i32 %r6, i32 %r7, i32 %r8, i32 %r9, i32 %r10, i32 %r11, i32 %r12, i32 %r13)
ret i32 %r14
}

define i32 @foo..I.I.I.I.I.I.I.I.I.I.I.I.I.I (i32 %a, i32 %b, i32 %c, i32 %d, i32 %e, i32 %f, i32 %g, i32 %h, i32 %i, i32 %j, i32 %k, i32 %l, i32 %m, i32 %n) {
LABEL_1:
%IDENT_1_a = alloca i32
store i32 %a, i32* %IDENT_1_a
%IDENT_1_b = alloca i32
store i32 %b, i32* %IDENT_1_b
%IDENT_1_c = alloca i32
store i32 %c, i32* %IDENT_1_c
%IDENT_1_d = alloca i32
store i32 %d, i32* %IDENT_1_d
%IDENT_1_e = alloca i32
store i32 %e, i32* %IDENT_1_e
%IDENT_1_f = alloca i32
store i32 %f, i32* %IDENT_1_f
%IDENT_1_g = alloca i32
store i32 %g, i32* %IDENT_1_g
%IDENT_1_h = alloca i32
store i32 %h, i32* %IDENT_1_h
%IDENT_1_i = alloca i32
store i32 %i, i32* %IDENT_1_i
%IDENT_1_j = alloca i32
store i32 %j, i32* %IDENT_1_j
%IDENT_1_k = alloca i32
store i32 %k, i32* %IDENT_1_k
%IDENT_1_l = alloca i32
store i32 %l, i32* %IDENT_1_l
%IDENT_1_m = alloca i32
store i32 %m, i32* %IDENT_1_m
%IDENT_1_n = alloca i32
store i32 %n, i32* %IDENT_1_n
%r15 = load i32, i32* %IDENT_1_a
%r16 = mul i32 2, %r15
%r17 = load i32, i32* %IDENT_1_b
%r18 = sdiv i32 %r17, 2
%r19 = add i32 %r16, %r18
%r20 = load i32, i32* %IDENT_1_c
%r21 = add i32 %r19, %r20
%r22 = load i32, i32* %IDENT_1_d
%r23 = add i32 %r21, %r22
%r24 = load i32, i32* %IDENT_1_e
%r25 = add i32 %r23, %r24
%r26 = load i32, i32* %IDENT_1_f
%r27 = add i32 %r25, %r26
%r28 = load i32, i32* %IDENT_1_g
%r29 = add i32 %r27, %r28
%r30 = load i32, i32* %IDENT_1_h
%r31 = add i32 %r29, %r30
%r32 = load i32, i32* %IDENT_1_i
%r33 = add i32 %r31, %r32
%r34 = load i32, i32* %IDENT_1_j
%r35 = sdiv i32 %r34, 2
%r36 = add i32 %r33, %r35
%r37 = load i32, i32* %IDENT_1_k
%r38 = add i32 %r36, %r37
%r39 = load i32, i32* %IDENT_1_l
%r40 = add i32 %r38, %r39
%r41 = load i32, i32* %IDENT_1_m
%r42 = add i32 %r40, %r41
%r43 = load i32, i32* %IDENT_1_n
%r44 = add i32 %r42, %r43
%r45 = srem i32 %r44, 10
%IDENT_0_r = alloca i32
store i32 %r45, i32* %IDENT_0_r
%r46 = load i32, i32* %IDENT_0_r
call void @printInt(i32 %r46)
%r47 = load i32, i32* %IDENT_0_r
ret i32 %r47
}

