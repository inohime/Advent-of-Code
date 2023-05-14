cd $1

missing_file_err="File for Advent of Code doesn't exists"

if [[ -d "$PWD/src/" ]]; then
    if [[ -f "$PWD/src/main.rs" ]]; then
        cargo run --release
    else
        echo $missing_file_err
        exit 1
    fi
else
    # not rust, get the extension by finding the file that has the same name as the directory
    # this will take the last file with the directory name!!

    aoc_day_path=$(find "$PWD" -type f -name "$1*")
    aoc_file_ext=""
    aoc_file=""

    for file in $aoc_day_path; do
        aoc_file_ext="${file##*.}"
        if [[ $aoc_file_ext == $file ]]; then
            unset -v aoc_file_ext
            continue
        fi
        aoc_file=$file
    done

    if [[ -z $aoc_file_ext ]]; then
        echo $missing_file_err
        exit 1
    fi

    case $aoc_file_ext in 
    "cpp")
        clang++ -std=c++2b -Wall -Wextra -pedantic-errors -O2 $aoc_file -o $1
        ./$1
        ;;

    "py")
        pypy3 $aoc_file
        ;;
    *)
        echo "Unknown file extension"
        ;;
    esac
fi

cd ..