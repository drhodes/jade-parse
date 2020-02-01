cargo clean
find ./ -type f -name "*~" -delete
rm -f perf.data perf.data.old flame.svg heaptrack*
echo all clean.
