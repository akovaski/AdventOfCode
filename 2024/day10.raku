sub MAIN($input) {
    my $file = open $input;
    my @map = $file.slurp.trim.lines>>.comb>>.Int;

    my @pos-tracking = [] xx 10;
    for 0..^@map.elems X 0..^@map[0].elems -> ($row, $col) {
        @pos-tracking[@map[$row][$col]].push(($row, $col).List);
    }

    my %on-possible-trail is default([]);
    my %trail-score-part2 is default(0);
    for 0..^@pos-tracking.elems -> $height {
        for @pos-tracking[$height].List -> ($row, $col) {
            if $height == 0 {
                %on-possible-trail{"$row;$col"} = set ("$row;$col",);
                %trail-score-part2{"$row;$col"} = 1;
            } else {
                for ((1,0), (-1, 0), (0, 1), (0, -1)) -> @neighbor-direction {
                    my @neighbor-position = ($row, $col) Z+ @neighbor-direction;
                    next if @neighbor-position.any < 0 or (@neighbor-position Z>= (@map.elems, @map[0].elems)).any;
                    next if @map[@neighbor-position[0]][@neighbor-position[1]] != $height - 1;
                    %on-possible-trail{"$row;$col"} ∪= %on-possible-trail{"{@neighbor-position[0]};{@neighbor-position[1]}"};
                    %trail-score-part2{"$row;$col"} += %trail-score-part2{"{@neighbor-position[0]};{@neighbor-position[1]}"};
                }
            }
        }
    }

    my $part1-solution = @pos-tracking[9].map({%on-possible-trail{"{$_[0]};{$_[1]}"}.elems}).sum;
    say "part 1: $part1-solution";

    my $part2-solution = @pos-tracking[9].map({%trail-score-part2{"{$_[0]};{$_[1]}"}}).sum;
    say "part 2: $part2-solution";
}