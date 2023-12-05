use v6;

sub MAIN($input) {
    my $file = open $input;

    grammar Almanac {
        token TOP {
            <seeds> "\n"
            <map>+ % "\n"+
            "\n"*
        }
        token seeds { 'seeds: ' (\d+)+ % \s "\n" }
        token map { <name> " map:\n" <mapping>+ % "\n" }
        token name { \S+ }
        rule mapping {<dest> <source> <range-len>}
        token dest { \d+ }
        token source { \d+ }
        token range-len { \d+ }
    }

    my $almanac = Almanac.parse($file.slurp);
    my @seeds = $almanac<seeds>[0]».Int.sort;
    say @seeds;
    my @vals = @seeds;
    for $almanac<map> -> $alm-map {
        my @map = $alm-map<mapping>.map({ ( .<dest>.Int, .<source>.Int, .<range-len>.Int ) }).sort(*.[1]);
        @vals= @vals.map({ mapValue(@map, $_) });
        # say "$alm-map<name> {@vals.raku}";
    }
    say "part 1: {[min] @vals}";

    my @val-ranges = $almanac<seeds>[0]».Int.rotor(2);
    for $almanac<map> -> $alm-map {
        my @map = $alm-map<mapping>.map({ ( .<dest>.Int, .<source>.Int, .<range-len>.Int ) }).sort(*.[1]);
        @val-ranges = @val-ranges.map({ mapRange(@map, $_) }).flat;
        # say "$alm-map<name> {@val-ranges.raku}";
    }
    say "part 2: {[min] @val-ranges>>[0]}";
}
sub mapValue(@map, $val) {
    for @map -> $map-v {
        if $map-v[1] > $val {
            return $val;
        } elsif $map-v[1] <= $val && $map-v[1] + $map-v[2] >= $val {
            return $map-v[0] + ($val - $map-v[1]);
        }
    }
    return $val;
}
sub mapRange(@map, @range) {
    my $start = @range[0];
    my $end = @range[0] + @range[1] - 1;
    my @outputs;
    for @map -> $mapping {
        my $mapping-start = $mapping[1];
        my $mapping-end = $mapping[1] + $mapping[2] - 1;
        my $mapping-dest = $mapping[0];
        next if $mapping-end < $start;
        last if $mapping-start > $end;
        
        if $mapping-start > $start {
            # values before or between mappings will be mapped 1-to-1
            @outputs.push((+$start, $mapping-start - $start));
            $start = $mapping-start;
        }
        my $this-end = $end min $mapping-end;
        @outputs.push(((+$start - $mapping-start) + $mapping-dest, ($this-end - $start + 1)));
        $start = $this-end+1;
    }
    if $start <= $end {
        # values after the last mapping will be mapped 1-to-1
        @outputs.push((+$start, $end - $start));
    }
    return @outputs;
}