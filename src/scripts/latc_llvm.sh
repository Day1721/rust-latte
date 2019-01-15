if [ $# -lt 1 ] 
then 
    echo "Argument expected"
    echo "Usage: latc_llvm <filename>"
    exit 1
fi

cargo run $1
llvm-link "${1%.*}.ll" lib/runtime.ll -o "${1%.*}.bc"