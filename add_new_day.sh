#!/bin/bash

DAY="$1"
[[ -z "$DAY" ]] && echo "make_new_day.sh dayXX" && exit 1
[[ -e "src/days/${DAY}a.rs" ]] && echo "day already exists" && exit 1
[[ -e "src/days/${DAY}b.rs" ]] && echo "day already exists" && exit 1

touch "inputs/${DAY}.txt"
touch "inputs/${DAY}.example.txt"
sed "s/DAY/${DAY}/g" templates/new_day.rs > "src/days/${DAY}a.rs"
sed "s/DAY/${DAY}/g" templates/new_day.rs > "src/days/${DAY}b.rs"
sed "s/DAY/${DAY}a/g" templates/new_day_bin.rs > "src/bin/days/${DAY}a.rs"
sed "s/DAY/${DAY}b/g" templates/new_day_bin.rs > "src/bin/days/${DAY}b.rs"
echo "pub mod ${DAY}a;
pub mod ${DAY}b;" >> src/days.rs
sed -i .swp "s#// (${DAY}#(${DAY}#g" src/lib.rs && rm src/lib.rs.swp
sed -i .swp "s#// c.bench_function(\"${DAY}#c.bench_function(\"${DAY}#g" benches/all-days.rs && rm benches/all-days.rs.swp
echo "
[[bin]]
name = \"${DAY}a\"
path = \"src/bin/days/${DAY}a.rs\"

[[bin]]
name = \"${DAY}b\"
path = \"src/bin/days/${DAY}b.rs\"" >> Cargo.toml
