
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        
@__STR__1 = private constant [5 x i8] c"true\00"
@__STR__0 = private constant [21 x i8] c"string concatenation\00"
@__STR__2 = private constant [6 x i8] c"false\00"

define i32 @main () {
LABEL_0:
%IDENT_0_x = alloca i32
store i32 56, i32* %IDENT_0_x
%IDENT_0_y = alloca i32
store i32 -23, i32* %IDENT_0_y
%r0 = load i32, i32* %IDENT_0_x
%r1 = load i32, i32* %IDENT_0_y
%r2 = add i32 %r0, %r1
call void @printInt(i32 %r2)
%r3 = load i32, i32* %IDENT_0_x
%r4 = load i32, i32* %IDENT_0_y
%r5 = sub i32 %r3, %r4
call void @printInt(i32 %r5)
%r6 = load i32, i32* %IDENT_0_x
%r7 = load i32, i32* %IDENT_0_y
%r8 = mul i32 %r6, %r7
call void @printInt(i32 %r8)
call void @printInt(i32 22)
call void @printInt(i32 0)
%r9 = load i32, i32* %IDENT_0_x
%r10 = load i32, i32* %IDENT_0_y
%r11 = sub i32 %r9, %r10
%r12 = load i32, i32* %IDENT_0_x
%r13 = load i32, i32* %IDENT_0_y
%r14 = add i32 %r12, %r13
%r15 = icmp sgt i32 %r11, %r14
call void @printBool..B(i1 %r15)
%r16 = load i32, i32* %IDENT_0_x
%r17 = load i32, i32* %IDENT_0_y
%r18 = sdiv i32 %r16, %r17
%r19 = load i32, i32* %IDENT_0_x
%r20 = load i32, i32* %IDENT_0_y
%r21 = mul i32 %r19, %r20
%r22 = icmp sle i32 %r18, %r21
call void @printBool..B(i1 %r22)
%r23 = getelementptr [21 x i8], [21 x i8]* @__STR__0, i32 0, i32 0
call void @printString(i8* %r23)
ret i32 0
}

define void @printBool..B (i1 %b) {
LABEL_1:
%IDENT_0_b = alloca i1
store i1 %b, i1* %IDENT_0_b
%r24 = load i1, i1* %IDENT_0_b
br i1 %r24, label %LABEL_2, label %LABEL_3
LABEL_2:
%r25 = getelementptr [5 x i8], [5 x i8]* @__STR__1, i32 0, i32 0
call void @printString(i8* %r25)
ret void
LABEL_3:
%r26 = getelementptr [6 x i8], [6 x i8]* @__STR__2, i32 0, i32 0
call void @printString(i8* %r26)
ret void
}

