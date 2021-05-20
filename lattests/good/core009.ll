
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        

define i32 @main () {
LABEL_0:
%r0 = call i32 @foo..()
%IDENT_0_x = alloca i32
store i32 %r0, i32* %IDENT_0_x
%r1 = load i32, i32* %IDENT_0_x
call void @printInt(i32 %r1)
ret i32 0
}

define i32 @foo.. () {
LABEL_1:
ret i32 10
}

