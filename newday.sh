cd data || exit


if [ $# -eq 0 ]
  then
    echo "ERROR: Day not specified"
    exit
fi

day=$1
$dir = "day$day"
mkdir "$dir"
touch "$dir/day$day.in"
touch "$dir/day$day.test.in"
touch "$dir/day$day.a.test.ans"
touch "$dir/day$day.b.test.ans"
echo 0 >> "$dir/day$day.a.test.ans"
echo 0 >> "$dir/day$day.b.test.ans"

cd ../src/days
cp day0_template.rs "day$day.rs"
