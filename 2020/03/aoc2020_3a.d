import std.stdio;
import std.file;
import std.algorithm;

void main()
{
    auto a = slurp!(string)("input.txt","%s");
    ulong x = 0;

    int hit_trees = 0;

    int valid = 0;
    foreach (line; a) {
        if (line[x % line.length] == '#') {
            hit_trees += 1;
        }
        x += 3;
    }
    writefln("tree count: %d", hit_trees);
}
