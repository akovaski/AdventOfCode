import std.stdio;
import std.file;
import std.algorithm;

void main()
{
    auto map = slurp!(string)("input.txt","%s");
    long a = calc_hit_trees(map, 1, 1);
    long b = calc_hit_trees(map, 3, 1);
    long c = calc_hit_trees(map, 5, 1);
    long d = calc_hit_trees(map, 7, 1);
    long e = calc_hit_trees(map, 1, 2);
    writefln("a: %d, b: %d, c: %d, d: %d, e: %d", a, b, c, d, e);
    writefln("tree count: %d", a*b*c*d*e);
}

int calc_hit_trees(string[] map, int x_delta, int y_delta) {
    ulong x = 0;
    ulong y = 0;
    int hit_trees = 0;

    while (y < map.length) {
        string line = map[y];
        if (line[x % line.length] == '#') {
            hit_trees += 1;
        }
        x += x_delta;
        y += y_delta;
    }
    return hit_trees;
}
