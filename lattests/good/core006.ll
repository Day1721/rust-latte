
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
store i32 0, i32* %IDENT_0_y
store i32 45, i32* %IDENT_0_x
store i32 -36, i32* %IDENT_0_y
%r0 = load i32, i32* %IDENT_0_x
call void @printInt(i32 %r0)
%r1 = load i32, i32* %IDENT_0_y
call void @printInt(i32 %r1)
ret i32 0
}

