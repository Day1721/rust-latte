
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        
@__STR__3 = private constant [6 x i8] c"false\00"
@__STR__0 = private constant [3 x i8] c"&&\00"
@__STR__2 = private constant [2 x i8] c"!\00"
@__STR__4 = private constant [5 x i8] c"true\00"
@__STR__1 = private constant [3 x i8] c"||\00"

define i32 @main () {
LABEL_0:
%r0 = getelementptr [3 x i8], [3 x i8]* @__STR__0, i32 0, i32 0
call void @printString(i8* %r0)
%r1 = call i1 @test..I(i32 -1)
br i1 %r1, label %LABEL_1, label %LABEL_2
LABEL_1:
%r2 = call i1 @test..I(i32 0)
br label %LABEL_2
LABEL_2:
%r3 = phi i1 [ false, %LABEL_0], [ %r2, %LABEL_1]
call void @printBool..B(i1 %r3)
%r4 = call i1 @test..I(i32 -2)
br i1 %r4, label %LABEL_3, label %LABEL_4
LABEL_3:
%r5 = call i1 @test..I(i32 1)
br label %LABEL_4
LABEL_4:
%r6 = phi i1 [ false, %LABEL_2], [ %r5, %LABEL_3]
call void @printBool..B(i1 %r6)
%r7 = call i1 @test..I(i32 3)
br i1 %r7, label %LABEL_5, label %LABEL_6
LABEL_5:
%r8 = call i1 @test..I(i32 -5)
br label %LABEL_6
LABEL_6:
%r9 = phi i1 [ false, %LABEL_4], [ %r8, %LABEL_5]
call void @printBool..B(i1 %r9)
%r10 = call i1 @test..I(i32 234234)
br i1 %r10, label %LABEL_7, label %LABEL_8
LABEL_7:
%r11 = call i1 @test..I(i32 21321)
br label %LABEL_8
LABEL_8:
%r12 = phi i1 [ false, %LABEL_6], [ %r11, %LABEL_7]
call void @printBool..B(i1 %r12)
%r13 = getelementptr [3 x i8], [3 x i8]* @__STR__1, i32 0, i32 0
call void @printString(i8* %r13)
%r14 = call i1 @test..I(i32 -1)
br i1 %r14, label %LABEL_10, label %LABEL_9
LABEL_9:
%r15 = call i1 @test..I(i32 0)
br label %LABEL_10
LABEL_10:
%r16 = phi i1 [ true, %LABEL_8], [ %r15, %LABEL_9]
call void @printBool..B(i1 %r16)
%r17 = call i1 @test..I(i32 -2)
br i1 %r17, label %LABEL_12, label %LABEL_11
LABEL_11:
%r18 = call i1 @test..I(i32 1)
br label %LABEL_12
LABEL_12:
%r19 = phi i1 [ true, %LABEL_10], [ %r18, %LABEL_11]
call void @printBool..B(i1 %r19)
%r20 = call i1 @test..I(i32 3)
br i1 %r20, label %LABEL_14, label %LABEL_13
LABEL_13:
%r21 = call i1 @test..I(i32 -5)
br label %LABEL_14
LABEL_14:
%r22 = phi i1 [ true, %LABEL_12], [ %r21, %LABEL_13]
call void @printBool..B(i1 %r22)
%r23 = call i1 @test..I(i32 234234)
br i1 %r23, label %LABEL_16, label %LABEL_15
LABEL_15:
%r24 = call i1 @test..I(i32 21321)
br label %LABEL_16
LABEL_16:
%r25 = phi i1 [ true, %LABEL_14], [ %r24, %LABEL_15]
call void @printBool..B(i1 %r25)
%r26 = getelementptr [2 x i8], [2 x i8]* @__STR__2, i32 0, i32 0
call void @printString(i8* %r26)
call void @printBool..B(i1 true)
call void @printBool..B(i1 false)
ret i32 0
}

define void @printBool..B (i1 %b) {
LABEL_17:
%IDENT_0_b = alloca i1
store i1 %b, i1* %IDENT_0_b
%r27 = load i1, i1* %IDENT_0_b
%r28 = sub i1 1, %r27
br i1 %r28, label %LABEL_18, label %LABEL_19
LABEL_18:
%r29 = getelementptr [6 x i8], [6 x i8]* @__STR__3, i32 0, i32 0
call void @printString(i8* %r29)
br label %LABEL_20
LABEL_19:
%r30 = getelementptr [5 x i8], [5 x i8]* @__STR__4, i32 0, i32 0
call void @printString(i8* %r30)
br label %LABEL_20
LABEL_20:
ret void
}

define i1 @test..I (i32 %i) {
LABEL_21:
%IDENT_0_i = alloca i32
store i32 %i, i32* %IDENT_0_i
%r31 = load i32, i32* %IDENT_0_i
call void @printInt(i32 %r31)
%r32 = load i32, i32* %IDENT_0_i
%r33 = icmp sgt i32 %r32, 0
ret i1 %r33
}

