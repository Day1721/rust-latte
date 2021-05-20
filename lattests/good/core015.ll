
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        

define i32 @main () {
LABEL_0:
%r0 = call i32 @ev..I(i32 17)
call void @printInt(i32 %r0)
ret i32 0
}

define i32 @ev..I (i32 %y) {
LABEL_1:
%IDENT_0_y = alloca i32
store i32 %y, i32* %IDENT_0_y
%r1 = load i32, i32* %IDENT_0_y
%r2 = icmp sgt i32 %r1, 0
br i1 %r2, label %LABEL_2, label %LABEL_3
LABEL_2:
%r3 = load i32, i32* %IDENT_0_y
%r4 = sub i32 %r3, 2
%r5 = call i32 @ev..I(i32 %r4)
ret i32 %r5
LABEL_3:
%r6 = load i32, i32* %IDENT_0_y
%r7 = icmp slt i32 %r6, 0
br i1 %r7, label %LABEL_5, label %LABEL_6
LABEL_5:
ret i32 0
LABEL_6:
ret i32 1
}

