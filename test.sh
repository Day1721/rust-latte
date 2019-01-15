make
for f in ./lattests/good/*.lat 
do
    ./latc_llvm $f
    echo "---------- GENERATED -------------"
    lli "${f%.*}.bc"
    echo "---------- EXPECTED --------------"
    cat "${f%.*}.output"
done

rm tst.bc

for f in ./lattests/bad/*.lat 
do
    cargo run $f
done
