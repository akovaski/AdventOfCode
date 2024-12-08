sub MAIN($input) {
    my $file = open $input;
    my @map = $file.slurp.lines>>.comb>>.Array.Array;
    my @start = find-start(@map);
    my @pos = @start;
    my @direction = (-1, 0);
    my %marks is SetHash = ();
    my $c = 0;
    loop {
        %marks{@pos.raku}++;
        my @step = @pos Z+ @direction;
        last if @step[0] < 0 || @step[1] >= @map.elems || @step[1] < 0 || @step[1] >= @map[@step[0]].elems;
        if @map[@step[0]; @step[1]] eq "#" {
            @direction = @direction[1], -1*@direction[0];
        } else {
            @pos = @step;
        }
    }
    my $part-one-solution = %marks.elems;
    say "part 1: $part-one-solution";
    say map-does-loop(@map, @start);
    my $part-two-solution = 0;
    for 0..^@map.elems -> $row {
        for 0..^@map[$row].elems -> $col {
            next if @map[$row; $col] (elem) ("^", "#");
            next if %marks{($row,$col).Array.raku}:!exists;
            @map[$row; $col] = "#";
            $part-two-solution++ if map-does-loop(@map, @start);
            @map[$row; $col] = ".";
        }
    }
    say "part 2: $part-two-solution";
}

sub find-start(@map) {
    for 0..^@map.elems -> $row {
        for 0..^@map[$row].elems -> $col {
            if @map[$row; $col] eq "^" {
                return ($row; $col);
            }
        }
    }
}

sub map-does-loop(@map, @start) {
    my @pos = @start;
    my @direction = (-1, 0);
    my %previous is SetHash = ();
    loop {
        my @step = @pos Z+ @direction;
        return False if @step[0] < 0 || @step[1] >= @map.elems || @step[1] < 0 || @step[1] >= @map[@step[0]].elems;
        if @map[@step[0]; @step[1]] eq "#" {
            @direction = @direction[1], -1*@direction[0];
            return True if %previous{(@pos, @direction).raku};
            %previous{(@pos, @direction).raku}++;
        } else {
            @pos = @step;
        }
    }

}