
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        

define i32 @main () {
LABEL_0:
%r0 = call i32 @readInt()
%IDENT_0_x = alloca i32
store i32 %r0, i32* %IDENT_0_x
%r1 = call i8* @readString()
%IDENT_0_y = alloca i8*
store i8* %r1, i8** %IDENT_0_y
%r2 = call i8* @readString()
%IDENT_0_z = alloca i8*
store i8* %r2, i8** %IDENT_0_z
%r3 = load i32, i32* %IDENT_0_x
%r4 = sub i32 %r3, 5
call void @printInt(i32 %r4)
%r5 = load i8*, i8** %IDENT_0_y
%r6 = load i8*, i8** %IDENT_0_z
%r7 = call i8* @concat(i8* %r5, i8* %r6)
call void @printString(i8* %r7)
ret i32 0
}

