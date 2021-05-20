
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        

define i32 @main () {
LABEL_0:
%IDENT_0_y = alloca i32
store i32 17, i32* %IDENT_0_y
br label %LABEL_1
LABEL_2:
%r0 = load i32, i32* %IDENT_0_y
%r1 = sub i32 %r0, 2
store i32 %r1, i32* %IDENT_0_y
br label %LABEL_1
LABEL_1:
%r2 = load i32, i32* %IDENT_0_y
%r3 = icmp sgt i32 %r2, 0
br i1 %r3, label %LABEL_2, label %LABEL_3
LABEL_3:
%r4 = load i32, i32* %IDENT_0_y
%r5 = icmp slt i32 %r4, 0
br i1 %r5, label %LABEL_4, label %LABEL_5
LABEL_4:
call void @printInt(i32 0)
ret i32 0
LABEL_5:
call void @printInt(i32 1)
ret i32 0
}

