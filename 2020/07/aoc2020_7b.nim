import strutils
import sequtils
import parseutils
import re
import options
import tables
import sets

proc parseBag(s: string): Option[(int, string)] =
    if s =~ re"(\d+) (.*)$":
        return some((parseInt(matches[0]), matches[1]))
    elif s == "no other bag":
        return none((int, string))
    else:
        echo s
        assert(false)

var bag_tree = initTable[string, seq[Option[(int, string)]]]()
for line in lines("input.txt"):
    var line_split = line.split(" contain ");
    assert line_split.len == 2
    line_split[0].removeSuffix('s')
    let parent_text = line_split[0]
    line_split[1].removeSuffix('.')
    var children = line_split[1].split(", ")
    children.apply(proc(s: var string) = s.removeSuffix('s'))
    let children_parsed = children.map(parseBag)
    bag_tree[parent_text] = children_parsed

proc bag_count(s: string): int =
    for c in bag_tree[s]:
        if c.isNone:
            continue
        result += c.get[0] * (c.get[1].bag_count() + 1)

echo bag_count("shiny gold bag")
