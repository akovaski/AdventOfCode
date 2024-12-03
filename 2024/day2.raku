use v6;

sub dampenedReports(@report) {
    my @reports = [];
    @reports.push(@report.List);
    for 0..^@report.elems {
        @reports.push(@report[0..^$_,$_+1..*].flat.List);
    }
    @reports.List
}

sub MAIN($input) {
    my $file = open $input;

    grammar ReportList {
        token TOP { <report>+%"\n" "\n"* }
        token report { <level>+%" " }
        token level { \d+ }
    }

    my $reportList = ReportList.parse($file.slurp);
    my @reports = $reportList<report>.map({.<level>.map({.Int}).List});
    my $part-one-solution = @reports.grep({given $_.List {
        ($_ eqv $_.sort.List or $_ eqv $_.sort.reverse.List) and
        so (1 <= (($_ Z- $_[1..*])».abs.all) <= 3)
    }}).elems;
    say "part 1: $part-one-solution";

    my $part-two-solution = @reports.grep({ dampenedReports($_.List).map({
        ($_ eqv $_.sort.List or $_ eqv $_.sort.reverse.List) and
        so (1 <= (($_ Z- $_[1..*])».abs.all) <= 3)
    }).any}).elems;
    say "part 2: $part-two-solution";
}