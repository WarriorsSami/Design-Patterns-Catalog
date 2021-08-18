#!/bin/sh

for dir in */; do
	touch "$dir/README.md"
done
