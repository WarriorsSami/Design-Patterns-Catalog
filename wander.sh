for dir in */; do
	echo "$dir"
	for subdir in $dir/; do
		echo "	$subdir"
	done
done
