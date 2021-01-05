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

var can_be_contained_in = initTable[string, seq[string]]()
for line in lines("input.txt"):
    var line_split = line.split(" contain ");
    assert line_split.len == 2
    line_split[0].removeSuffix('s')
    let parent_text = line_split[0]
    line_split[1].removeSuffix('.')
    var children = line_split[1].split(", ")
    children.apply(proc(s: var string) = s.removeSuffix('s'))
    let children_parsed = children.map(parseBag)

    for c in children_parsed:
        if c.isSome:
            let sub_bag = c.get[1]
            var outer_bags = can_be_contained_in.getOrDefault(sub_bag)
            outer_bags.add(parent_text)
            can_be_contained_in[sub_bag] = outer_bags

var bags_to_check = @["shiny gold bag"]
var valid_bags = HashSet[string]()
while bags_to_check.len != 0:
    let btc = bags_to_check.pop()
    for parent_bag in can_be_contained_in.getOrDefault(btc):
        if not valid_bags.contains(parent_bag):
            valid_bags.incl(parent_bag)
            bags_to_check.add(parent_bag)

echo valid_bags.len

