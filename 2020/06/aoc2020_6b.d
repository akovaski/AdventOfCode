import std.stdio;
import std.string;
import std.file;
import std.algorithm;
import std.array;
import std.conv;

void main()
{
    auto input_file = readText("input.txt").splitLines();

    ulong sum_all_groups;
    int[char] group_answers;
    int group_count = 0;

    foreach(i, line; input_file) {
        if (line == "") {
            sum_all_groups += group_answers.byValue().filter!(v => v == group_count).count;
            group_answers.clear;
            group_count = 0;
        } else {
            foreach(a; line) {
                group_answers[a] += 1;
            }
            group_count += 1;
        }
    }

    writefln("sum of all group answer counts: %d", sum_all_groups);
}
