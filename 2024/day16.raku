sub MAIN($input) {
    my $file = open $input;
    my @map = $file.slurp.trim.lines>>.comb>>.List;
    my @start-pos = find-pos(@map, "S");
    my @end-pos = find-pos(@map, "E");
    
    my %pos-dir-scores;
    my @pos = @start-pos;
    my @dir = (0, 1);

    search-part1(@map, @start-pos, @dir, %pos-dir-scores);
    my $part1-solution = Nil;
    for ((-1, 0), (1, 0), (0, -1), (0, 1)) -> @end-dir {
        $part1-solution min= %pos-dir-scores{pd-index(@end-pos, @end-dir)}
    }
    say "part1 solution: $part1-solution";

    my $part2-solution = count-part2(%pos-dir-scores, @start-pos, @end-pos, $part1-solution);
    say "part2 solution: $part2-solution";
}

sub find-pos(@map, $needle) {
    for ^@map.elems X ^@map[0].elems -> ($row, $col) {
        return ($row, $col) if @map[$row][$col] eq $needle;
    }
    return Nil;
}

sub search-part1(@map, @start-pos, @start-dir, %pos-dir-scores) {
    my @search-queue = ((@start-pos, @start-dir, 0),);
    while @search-queue.elems != 0 {
        my (@pos, @dir, $score) Z= @search-queue.shift.Seq;
        next unless 0 <= @pos[0] < @map.elems;
        next unless 0 <= @pos[1] < @map[0].elems;
        next if @map[@pos[0]][@pos[1]] eq "#";
        next if %pos-dir-scores{pd-index(@pos, @dir)}:exists && $score >= %pos-dir-scores{pd-index(@pos, @dir)};
        my @reverse-dir = @dir.map(* * -1);
        next if %pos-dir-scores{pd-index(@pos, @reverse-dir)}:exists && $score - 2000 > %pos-dir-scores{pd-index(@pos, @reverse-dir)};
        %pos-dir-scores{pd-index(@pos, @dir)} = $score;
        # forward
        my @moved = @pos Z+ @dir;
        # clockwise
        my @cw-dir = (-@dir[1], @dir[0]);
        # counter-clockwise
        my @ccw-dir = (@dir[1], -@dir[0]);
        @search-queue.append(
            (@moved, @dir, $score + 1),
            (@pos, @cw-dir, $score + 1000),
            (@pos, @ccw-dir, $score + 1000)
        );
    }
}

sub pd-index(@pos, @dir) {
    "{@pos[0]};{@pos[1]}#{@dir[0]};{@dir[1]}"
}

sub count-part2(%pos-dir-scores, @start-pos, @end-pos, $min-score) {
    my @search-queue;
    for ((-1, 0), (1, 0), (0, -1), (0, 1)) -> @end-dir {
        @search-queue.push((@end-pos, @end-dir, $min-score)) if %pos-dir-scores{pd-index(@end-pos, @end-dir)}:exists && $min-score == %pos-dir-scores{pd-index(@end-pos, @end-dir)};
    }
    my %on-path is SetHash;
    while @search-queue.elems != 0 {
        my (@pos, @dir, $score) Z= @search-queue.shift.Seq;
        next if %pos-dir-scores{pd-index(@pos, @dir)}:!exists;
        next if %pos-dir-scores{pd-index(@pos, @dir)} != $score;
        next if %on-path{pd-index(@pos, @dir)}:exists;
        %on-path{pd-index(@pos, @dir)}++;
        # rev-forward
        my @moved = @pos Z- @dir;
        # rev-clockwise
        my @cw-dir = (@dir[1], -@dir[0]);
        # rev-counter-clockwise
        my @ccw-dir = (-@dir[1], @dir[0]);
        @search-queue.append(
            (@moved, @dir, $score - 1),
            (@pos, @cw-dir, $score - 1000),
            (@pos, @ccw-dir, $score - 1000)
        );
    }
    return %on-path.keys.map(*.split("#")[0]).Set.elems;
}