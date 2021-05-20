
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        
@__STR__0 = private constant [4 x i8] c"apa\00"
@__STR__2 = private constant [6 x i8] c"false\00"
@__STR__1 = private constant [5 x i8] c"true\00"

define i32 @main () {
LABEL_0:
%IDENT_0_x = alloca i32
store i32 4, i32* %IDENT_0_x
%r0 = load i32, i32* %IDENT_0_x
%r1 = icmp sle i32 3, %r0
br i1 %r1, label %LABEL_1, label %LABEL_2
LABEL_1:
br i1 true, label %LABEL_3, label %LABEL_4
LABEL_3:
br label %LABEL_4
LABEL_4:
%r2 = phi i1 [ false, %LABEL_1], [ true, %LABEL_3]
br label %LABEL_2
LABEL_2:
%r3 = phi i1 [ false, %LABEL_0], [ %r2, %LABEL_4]
br i1 %r3, label %LABEL_5, label %LABEL_6
LABEL_5:
call void @printBool..B(i1 true)
br label %LABEL_7
LABEL_6:
%r4 = getelementptr [4 x i8], [4 x i8]* @__STR__0, i32 0, i32 0
call void @printString(i8* %r4)
br label %LABEL_7
LABEL_7:
br i1 true, label %LABEL_9, label %LABEL_8
LABEL_8:
%r5 = call i1 @dontCallMe..I(i32 1)
br label %LABEL_9
LABEL_9:
%r6 = phi i1 [ true, %LABEL_7], [ %r5, %LABEL_8]
call void @printBool..B(i1 %r6)
br i1 false, label %LABEL_10, label %LABEL_11
LABEL_10:
%r7 = call i1 @dontCallMe..I(i32 2)
br label %LABEL_11
LABEL_11:
%r8 = phi i1 [ false, %LABEL_9], [ %r7, %LABEL_10]
call void @printBool..B(i1 %r8)
%r9 = load i32, i32* %IDENT_0_x
%r10 = icmp eq i32 4, %r9
br i1 %r10, label %LABEL_12, label %LABEL_13
LABEL_12:
br i1 true, label %LABEL_14, label %LABEL_15
LABEL_14:
br label %LABEL_15
LABEL_15:
%r11 = phi i1 [ false, %LABEL_12], [ true, %LABEL_14]
br label %LABEL_13
LABEL_13:
%r12 = phi i1 [ false, %LABEL_11], [ %r11, %LABEL_15]
call void @printBool..B(i1 %r12)
%r13 = call i1 @implies..B.B(i1 false, i1 false)
call void @printBool..B(i1 %r13)
%r14 = call i1 @implies..B.B(i1 false, i1 true)
call void @printBool..B(i1 %r14)
%r15 = call i1 @implies..B.B(i1 true, i1 false)
call void @printBool..B(i1 %r15)
%r16 = call i1 @implies..B.B(i1 true, i1 true)
call void @printBool..B(i1 %r16)
ret i32 0
}

define i1 @dontCallMe..I (i32 %x) {
LABEL_16:
%IDENT_1_x = alloca i32
store i32 %x, i32* %IDENT_1_x
%r17 = load i32, i32* %IDENT_1_x
call void @printInt(i32 %r17)
ret i1 true
}

define void @printBool..B (i1 %b) {
LABEL_17:
%IDENT_0_b = alloca i1
store i1 %b, i1* %IDENT_0_b
%r18 = load i1, i1* %IDENT_0_b
br i1 %r18, label %LABEL_18, label %LABEL_19
LABEL_18:
%r19 = getelementptr [5 x i8], [5 x i8]* @__STR__1, i32 0, i32 0
call void @printString(i8* %r19)
br label %LABEL_20
LABEL_19:
%r20 = getelementptr [6 x i8], [6 x i8]* @__STR__2, i32 0, i32 0
call void @printString(i8* %r20)
br label %LABEL_20
LABEL_20:
ret void
}

define i1 @implies..B.B (i1 %x, i1 %y) {
LABEL_21:
%IDENT_1_x = alloca i1
store i1 %x, i1* %IDENT_1_x
%IDENT_0_y = alloca i1
store i1 %y, i1* %IDENT_0_y
%r21 = load i1, i1* %IDENT_1_x
%r22 = sub i1 1, %r21
br i1 %r22, label %LABEL_23, label %LABEL_22
LABEL_22:
%r23 = load i1, i1* %IDENT_1_x
%r24 = load i1, i1* %IDENT_0_y
%r25 = icmp eq i1 %r23, %r24
br label %LABEL_23
LABEL_23:
%r26 = phi i1 [ true, %LABEL_21], [ %r25, %LABEL_22]
ret i1 %r26
}

