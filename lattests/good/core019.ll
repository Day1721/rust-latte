
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        
@__STR__0 = private constant [4 x i8] c"foo\00"

define i32 @main () {
LABEL_0:
%IDENT_0_i = alloca i32
store i32 78, i32* %IDENT_0_i
%IDENT_1_i = alloca i32
store i32 1, i32* %IDENT_1_i
%r0 = load i32, i32* %IDENT_1_i
call void @printInt(i32 %r0)
%r1 = load i32, i32* %IDENT_0_i
call void @printInt(i32 %r1)
br label %LABEL_1
LABEL_2:
%r2 = load i32, i32* %IDENT_0_i
%r3 = sub i32 %r2, 1
store i32 %r3, i32* %IDENT_0_i
%r4 = load i32, i32* %IDENT_0_i
call void @printInt(i32 %r4)
%r5 = load i32, i32* %IDENT_0_i
%r6 = add i32 %r5, 7
%IDENT_2_i = alloca i32
store i32 %r6, i32* %IDENT_2_i
%r7 = load i32, i32* %IDENT_2_i
call void @printInt(i32 %r7)
br label %LABEL_1
LABEL_1:
%r8 = load i32, i32* %IDENT_0_i
%r9 = icmp sgt i32 %r8, 76
br i1 %r9, label %LABEL_2, label %LABEL_3
LABEL_3:
%r10 = load i32, i32* %IDENT_0_i
call void @printInt(i32 %r10)
%r11 = load i32, i32* %IDENT_0_i
%r12 = icmp sgt i32 %r11, 4
br i1 %r12, label %LABEL_4, label %LABEL_5
LABEL_4:
%IDENT_3_i = alloca i32
store i32 4, i32* %IDENT_3_i
%r13 = load i32, i32* %IDENT_3_i
call void @printInt(i32 %r13)
br label %LABEL_6
LABEL_5:
%r14 = getelementptr [4 x i8], [4 x i8]* @__STR__0, i32 0, i32 0
call void @printString(i8* %r14)
br label %LABEL_6
LABEL_6:
%r15 = load i32, i32* %IDENT_0_i
call void @printInt(i32 %r15)
ret i32 0
}

