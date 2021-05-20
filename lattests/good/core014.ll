
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        

define i32 @main () {
LABEL_0:
%IDENT_0_lo = alloca i32
store i32 0, i32* %IDENT_0_lo
%IDENT_0_hi = alloca i32
store i32 0, i32* %IDENT_0_hi
%IDENT_0_mx = alloca i32
store i32 0, i32* %IDENT_0_mx
store i32 1, i32* %IDENT_0_lo
%r0 = load i32, i32* %IDENT_0_lo
store i32 %r0, i32* %IDENT_0_hi
store i32 5000000, i32* %IDENT_0_mx
%r1 = load i32, i32* %IDENT_0_lo
call void @printInt(i32 %r1)
br label %LABEL_1
LABEL_2:
%r2 = load i32, i32* %IDENT_0_hi
call void @printInt(i32 %r2)
%r3 = load i32, i32* %IDENT_0_lo
%r4 = load i32, i32* %IDENT_0_hi
%r5 = add i32 %r3, %r4
store i32 %r5, i32* %IDENT_0_hi
%r6 = load i32, i32* %IDENT_0_hi
%r7 = load i32, i32* %IDENT_0_lo
%r8 = sub i32 %r6, %r7
store i32 %r8, i32* %IDENT_0_lo
br label %LABEL_1
LABEL_1:
%r9 = load i32, i32* %IDENT_0_hi
%r10 = load i32, i32* %IDENT_0_mx
%r11 = icmp slt i32 %r9, %r10
br i1 %r11, label %LABEL_2, label %LABEL_3
LABEL_3:
ret i32 0
}

