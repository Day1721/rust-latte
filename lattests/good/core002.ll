
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        
@__STR__0 = private constant [4 x i8] c"foo\00"

define i32 @main () {
LABEL_0:
call void @foo..()
ret i32 0
}

define void @foo.. () {
LABEL_1:
%r0 = getelementptr [4 x i8], [4 x i8]* @__STR__0, i32 0, i32 0
call void @printString(i8* %r0)
ret void
}

