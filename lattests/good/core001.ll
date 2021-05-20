
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        
@__STR__2 = private constant [9 x i8] c"/* world\00"
@__STR__1 = private constant [9 x i8] c"hello */\00"
@__STR__0 = private constant [2 x i8] c"=\00"
@__STR__3 = private constant [1 x i8] c"\00"

define i32 @main () {
LABEL_0:
%r0 = call i32 @fac..I(i32 10)
call void @printInt(i32 %r0)
%r1 = call i32 @rfac..I(i32 10)
call void @printInt(i32 %r1)
%r2 = call i32 @mfac..I(i32 10)
call void @printInt(i32 %r2)
%r3 = call i32 @ifac..I(i32 10)
call void @printInt(i32 %r3)
%IDENT_0_r = alloca i8*
store i8* null, i8** %IDENT_0_r
%IDENT_0_n = alloca i32
store i32 10, i32* %IDENT_0_n
%IDENT_1_r = alloca i32
store i32 1, i32* %IDENT_1_r
br label %LABEL_1
LABEL_2:
%r4 = load i32, i32* %IDENT_1_r
%r5 = load i32, i32* %IDENT_0_n
%r6 = mul i32 %r4, %r5
store i32 %r6, i32* %IDENT_1_r
%r7 = load i32, i32* %IDENT_0_n
%r8 = sub i32 %r7, 1
store i32 %r8, i32* %IDENT_0_n
br label %LABEL_1
LABEL_1:
%r9 = load i32, i32* %IDENT_0_n
%r10 = icmp sgt i32 %r9, 0
br i1 %r10, label %LABEL_2, label %LABEL_3
LABEL_3:
%r11 = load i32, i32* %IDENT_1_r
call void @printInt(i32 %r11)
%r12 = getelementptr [2 x i8], [2 x i8]* @__STR__0, i32 0, i32 0
%r13 = call i8* @repStr..S.I(i8* %r12, i32 60)
call void @printString(i8* %r13)
%r14 = getelementptr [9 x i8], [9 x i8]* @__STR__1, i32 0, i32 0
call void @printString(i8* %r14)
%r15 = getelementptr [9 x i8], [9 x i8]* @__STR__2, i32 0, i32 0
call void @printString(i8* %r15)
ret i32 0
}

define i32 @fac..I (i32 %a) {
LABEL_4:
%IDENT_0_a = alloca i32
store i32 %a, i32* %IDENT_0_a
%IDENT_2_r = alloca i32
store i32 0, i32* %IDENT_2_r
%IDENT_1_n = alloca i32
store i32 0, i32* %IDENT_1_n
store i32 1, i32* %IDENT_2_r
%r16 = load i32, i32* %IDENT_0_a
store i32 %r16, i32* %IDENT_1_n
br label %LABEL_5
LABEL_6:
%r17 = load i32, i32* %IDENT_2_r
%r18 = load i32, i32* %IDENT_1_n
%r19 = mul i32 %r17, %r18
store i32 %r19, i32* %IDENT_2_r
%r20 = load i32, i32* %IDENT_1_n
%r21 = sub i32 %r20, 1
store i32 %r21, i32* %IDENT_1_n
br label %LABEL_5
LABEL_5:
%r22 = load i32, i32* %IDENT_1_n
%r23 = icmp sgt i32 %r22, 0
br i1 %r23, label %LABEL_6, label %LABEL_7
LABEL_7:
%r24 = load i32, i32* %IDENT_2_r
ret i32 %r24
}

define i32 @rfac..I (i32 %n) {
LABEL_8:
%IDENT_2_n = alloca i32
store i32 %n, i32* %IDENT_2_n
%r25 = load i32, i32* %IDENT_2_n
%r26 = icmp eq i32 %r25, 0
br i1 %r26, label %LABEL_9, label %LABEL_10
LABEL_9:
ret i32 1
LABEL_10:
%r27 = load i32, i32* %IDENT_2_n
%r28 = load i32, i32* %IDENT_2_n
%r29 = sub i32 %r28, 1
%r30 = call i32 @rfac..I(i32 %r29)
%r31 = mul i32 %r27, %r30
ret i32 %r31
}

define i32 @mfac..I (i32 %n) {
LABEL_12:
%IDENT_2_n = alloca i32
store i32 %n, i32* %IDENT_2_n
%r32 = load i32, i32* %IDENT_2_n
%r33 = icmp eq i32 %r32, 0
br i1 %r33, label %LABEL_13, label %LABEL_14
LABEL_13:
ret i32 1
LABEL_14:
%r34 = load i32, i32* %IDENT_2_n
%r35 = load i32, i32* %IDENT_2_n
%r36 = sub i32 %r35, 1
%r37 = call i32 @nfac..I(i32 %r36)
%r38 = mul i32 %r34, %r37
ret i32 %r38
}

define i32 @nfac..I (i32 %n) {
LABEL_16:
%IDENT_2_n = alloca i32
store i32 %n, i32* %IDENT_2_n
%r39 = load i32, i32* %IDENT_2_n
%r40 = icmp ne i32 %r39, 0
br i1 %r40, label %LABEL_17, label %LABEL_18
LABEL_17:
%r41 = load i32, i32* %IDENT_2_n
%r42 = sub i32 %r41, 1
%r43 = call i32 @mfac..I(i32 %r42)
%r44 = load i32, i32* %IDENT_2_n
%r45 = mul i32 %r43, %r44
ret i32 %r45
LABEL_18:
ret i32 1
}

define i32 @ifac..I (i32 %n) {
LABEL_20:
%IDENT_2_n = alloca i32
store i32 %n, i32* %IDENT_2_n
%r46 = load i32, i32* %IDENT_2_n
%r47 = call i32 @ifac2f..I.I(i32 1, i32 %r46)
ret i32 %r47
}

define i32 @ifac2f..I.I (i32 %l, i32 %h) {
LABEL_21:
%IDENT_0_l = alloca i32
store i32 %l, i32* %IDENT_0_l
%IDENT_0_h = alloca i32
store i32 %h, i32* %IDENT_0_h
%r48 = load i32, i32* %IDENT_0_l
%r49 = load i32, i32* %IDENT_0_h
%r50 = icmp eq i32 %r48, %r49
br i1 %r50, label %LABEL_22, label %LABEL_24
LABEL_22:
%r51 = load i32, i32* %IDENT_0_l
ret i32 %r51
LABEL_24:
%r52 = load i32, i32* %IDENT_0_l
%r53 = load i32, i32* %IDENT_0_h
%r54 = icmp sgt i32 %r52, %r53
br i1 %r54, label %LABEL_25, label %LABEL_27
LABEL_25:
ret i32 1
LABEL_27:
%IDENT_0_m = alloca i32
store i32 0, i32* %IDENT_0_m
%r55 = load i32, i32* %IDENT_0_l
%r56 = load i32, i32* %IDENT_0_h
%r57 = add i32 %r55, %r56
%r58 = sdiv i32 %r57, 2
store i32 %r58, i32* %IDENT_0_m
%r59 = load i32, i32* %IDENT_0_l
%r60 = load i32, i32* %IDENT_0_m
%r61 = call i32 @ifac2f..I.I(i32 %r59, i32 %r60)
%r62 = load i32, i32* %IDENT_0_m
%r63 = add i32 %r62, 1
%r64 = load i32, i32* %IDENT_0_h
%r65 = call i32 @ifac2f..I.I(i32 %r63, i32 %r64)
%r66 = mul i32 %r61, %r65
ret i32 %r66
}

define i8* @repStr..S.I (i8* %s, i32 %n) {
LABEL_28:
%IDENT_0_s = alloca i8*
store i8* %s, i8** %IDENT_0_s
%IDENT_2_n = alloca i32
store i32 %n, i32* %IDENT_2_n
%r67 = getelementptr [1 x i8], [1 x i8]* @__STR__3, i32 0, i32 0
%IDENT_3_r = alloca i8*
store i8* %r67, i8** %IDENT_3_r
%IDENT_0_i = alloca i32
store i32 0, i32* %IDENT_0_i
br label %LABEL_29
LABEL_30:
%r68 = load i8*, i8** %IDENT_3_r
%r69 = load i8*, i8** %IDENT_0_s
%r70 = call i8* @concat(i8* %r68, i8* %r69)
store i8* %r70, i8** %IDENT_3_r
%r71 = load i32, i32* %IDENT_0_i
%r72 = add i32 %r71, 1
store i32 %r72, i32* %IDENT_0_i
br label %LABEL_29
LABEL_29:
%r73 = load i32, i32* %IDENT_0_i
%r74 = load i32, i32* %IDENT_2_n
%r75 = icmp slt i32 %r73, %r74
br i1 %r75, label %LABEL_30, label %LABEL_31
LABEL_31:
%r76 = load i8*, i8** %IDENT_3_r
ret i8* %r76
}

