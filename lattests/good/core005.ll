
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        

define i32 @main () {
LABEL_0:
%IDENT_0_x = alloca i32
store i32 0, i32* %IDENT_0_x
%IDENT_0_y = alloca i32
store i32 56, i32* %IDENT_0_y
%r0 = load i32, i32* %IDENT_0_y
%r1 = add i32 %r0, 45
%r2 = icmp sle i32 %r1, 2
br i1 %r2, label %LABEL_1, label %LABEL_2
LABEL_1:
store i32 1, i32* %IDENT_0_x
br label %LABEL_3
LABEL_2:
store i32 2, i32* %IDENT_0_x
br label %LABEL_3
LABEL_3:
%r3 = load i32, i32* %IDENT_0_x
call void @printInt(i32 %r3)
ret i32 0
}

