use v6;

sub MAIN($input) {
    my $file = open $input;

    grammar LocationList {
        token TOP { <row>+%"\n" "\n"* }
        token row { <left=.id> " "+ <right=.id> }
        token id { \d+ }
    }

    my $locations = LocationList.parse($file.slurp);
    my @rows = $locations<row>.map({ (.<left>.Int, .<right>.Int)});
    my $part-one-solution = (@rows[*;0].sort Z- @rows[*;1].sort)».abs.sum;
    say "part 1: $part-one-solution";

    my $rbag = bag(@rows[*;1].sort);
    my $part-two-solution = @rows[*;0].map({ $_ * $rbag{$_}}).sum;
    say "part 2: $part-two-solution";
}