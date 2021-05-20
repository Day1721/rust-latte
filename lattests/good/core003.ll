
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        

define i32 @f.. () {
LABEL_0:
ret i32 0
}

define i32 @g.. () {
LABEL_1:
ret i32 0
}

define void @p.. () {
LABEL_2:
ret void
}

define i32 @main () {
LABEL_3:
call void @p..()
ret i32 0
}

