import std.stdio;
import std.string;
import std.file;
import std.algorithm;
import std.array;
import std.conv;

void main()
{
    auto input_file = readText("input.txt").splitLines();

    bool[char][] all_group_answers;
    bool[char] group_answers;

    foreach(i, line; input_file) {
        if (line == "") {
            all_group_answers ~= group_answers.dup;
            group_answers.clear;
        } else {
            foreach(a; line) {
                group_answers[a] = true;
            }
        }
    }

    ulong sum_all_groups = all_group_answers.map!(ga => ga.length).sum;
    writefln("sum of all group answer counts: %d", sum_all_groups);
}
