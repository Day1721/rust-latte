
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        

define i32 @main () {
LABEL_0:
%r0 = call i32 @fac..I(i32 5)
call void @printInt(i32 %r0)
ret i32 0
}

define i32 @fac..I (i32 %a) {
LABEL_1:
%IDENT_0_a = alloca i32
store i32 %a, i32* %IDENT_0_a
%IDENT_0_r = alloca i32
store i32 0, i32* %IDENT_0_r
%IDENT_0_n = alloca i32
store i32 0, i32* %IDENT_0_n
store i32 1, i32* %IDENT_0_r
%r1 = load i32, i32* %IDENT_0_a
store i32 %r1, i32* %IDENT_0_n
br label %LABEL_2
LABEL_3:
%r2 = load i32, i32* %IDENT_0_r
%r3 = load i32, i32* %IDENT_0_n
%r4 = mul i32 %r2, %r3
store i32 %r4, i32* %IDENT_0_r
%r5 = load i32, i32* %IDENT_0_n
%r6 = sub i32 %r5, 1
store i32 %r6, i32* %IDENT_0_n
br label %LABEL_2
LABEL_2:
%r7 = load i32, i32* %IDENT_0_n
%r8 = icmp sgt i32 %r7, 0
br i1 %r8, label %LABEL_3, label %LABEL_4
LABEL_4:
%r9 = load i32, i32* %IDENT_0_r
ret i32 %r9
}

