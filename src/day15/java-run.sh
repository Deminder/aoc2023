#!/bin/bash
set -e

javac -d classes Day15.java
java --class-path classes Day15
