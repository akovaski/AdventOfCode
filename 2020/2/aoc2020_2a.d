import std.stdio;
import std.file;
import std.algorithm;

void main()
{
    auto a = slurp!(int, int, char, string)("input.txt","%s-%s %s: %s");

    int valid = 0;
    foreach (pline; a) {
        auto min = pline[0];
        auto max = pline[1];
        auto rule_char = pline[2];
        auto pass = pline[3];

        auto char_count = pass.count(rule_char);
        if (char_count >= min && char_count <= max) {
            valid += 1;
        }
    }
    writefln("valid count: %d", valid);
}
