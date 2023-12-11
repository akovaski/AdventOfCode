use v6;

sub MAIN($input) {
    my $file = open $input;

    my @map = $file.lines».comb».Array;
    my @galaxies-original = @map».grep("#", :k).grep(*.elems > 0, :kv).rotor(2).map({($_[0] X $_[1]).Slip});
    my $distances-original = @galaxies-original.List.combinations(2).map({($_[0] Z- $_[1])».abs.sum}).sum;

    # contract the universe
    @map = @map.map: {$_.all eq '.' ?? slip() !! $_};
    @map = [Z] @map;
    @map = @map.map: {$_.all eq '.' ?? slip() !! $_};
    @map = [Z] @map;

    my @galaxies = @map».grep("#", :k).grep(*.elems > 0, :kv).rotor(2).map({($_[0] X $_[1]).Slip});
    my $distances-contracted = @galaxies.List.combinations(2).map({($_[0] Z- $_[1])».abs.sum}).sum;

    my $distances-twice-expanded = ($distances-original - $distances-contracted) * 2 + $distances-contracted;
    say "part 1: $distances-twice-expanded";
    my $distances-many-expanded = ($distances-original - $distances-contracted) * 1000000 + $distances-contracted;
    say "part 2: $distances-many-expanded";
}