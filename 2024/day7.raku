sub MAIN($input) {
    grammar Input {
        token TOP { <line>+%"\n" "\n"* }
        token line { (\d+) ": " (\d+)+%" " }
    }
    my $parsed = Input.parsefile($input);
    my @lines is List = $parsed<line>.map({(.[0].Int, .[1]>>.Int.List)});
    my $part1-solution = 0;
    my $part2-solution = 0;
    for @lines -> ($test-value, @values) {
        my $matches = find-matching-equation-part1($test-value, @values);
        $part1-solution += $test-value if find-matching-equation-part1($test-value, @values);
        $part2-solution += $test-value if find-matching-equation-part2($test-value, @values);
    }
    say "part 1: $part1-solution";
    say "part 2: $part2-solution";
}

sub find-matching-equation-part1($test-value, @values) {
    if @values.elems == 1 {
        return $test-value == @values[0];
    }
    my $last = @values[*-1];
    if $test-value - $last >= 0 {
        return True if find-matching-equation-part1($test-value - $last, @values[0..*-2]);
    }
    if $test-value %% $last {
        return True if find-matching-equation-part1($test-value div $last, @values[0..*-2]);
    }
    return False;
}

sub find-matching-equation-part2($test-value, @values) {
    if @values.elems == 1 {
        return $test-value == @values[0];
    }
    my $last = @values[*-1];
    return False if $last > $test-value;
    if $test-value - $last >= 0 {
        return True if find-matching-equation-part2($test-value - $last, @values[0..*-2]);
    }
    if $test-value %% $last {
        return True if find-matching-equation-part2($test-value div $last, @values[0..*-2]);
    }
    if $test-value.Str.substr(*-$last.Str.chars, *) eq $last.Str {
        return True if find-matching-equation-part2($test-value.Str.substr(0,*-$last.Str.chars).Int, @values[0..*-2]);
    }
    return False;
}