
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        

define i32 @main () {
LABEL_0:
call void @p..()
call void @printInt(i32 1)
ret i32 0
}

define void @p.. () {
LABEL_1:
ret void
}

