#!bin/bash
cd ..
echo "# Compiling Release ..."
cargo build --release

cd ./benchmark
echo "# Starting Benchmark ..."
for n in {4..5}
do
  echo "# Working in [N=$n] "
  ./../target/release/tsp_machine bench:quick $n 10 times_n$n.part_a.csv > log_n$n.part_a.txt &
  ./../target/release/tsp_machine bench:quick $n 10 times_n$n.part_b.csv > log_n$n.part_b.txt &
 # ./../target/release/tsp_machine bench:quick $n 10 times_n$n.part_c.csv > log_n$n.part_c.txt &
  wait
done