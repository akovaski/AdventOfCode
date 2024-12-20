sub MAIN($input) {
    grammar Input {
        token TOP { <towel>+%", " "\n\n" <design>+%"\n" "\n"* }
        token towel { \w+ }
        token design { \w+ }
    }
    my $parsed = Input.parsefile($input);
    my @towels = $parsed<towel>>>.Str;
    my %towels = @towels.Set;
    my $max-towel-length = @towels.map(*.chars).max;
    my @designs = $parsed<design>>>.Str;
    my %valid-suffixes is SetHash = @towels.SetHash;
    my %valid-prefixes is SetHash = @towels.SetHash;
    my $part1-solution = @designs.map({is-valid-design(%towels, $max-towel-length, $_, ().SetHash, ().SetHash)}).sum;
    say "part1 solution: $part1-solution";

    my $part2-solution = @designs.map({count-valid-design(%towels, $max-towel-length, $_, ().Hash)}).sum;
    say "part2-solution: $part2-solution";
}

sub is-valid-design(%towels, $max-towel-length, $design, %valid-suffixes, %invalid-suffixes) {
    for min($max-towel-length, $design.chars)...1 {
        my $prefix = $design.substr(0,$_);
        next if %towels{$prefix}:!exists;
        return True if $_ == $design.chars;

        my $suffix = $design.substr($_, *);
        return True if %valid-suffixes{$suffix.chars};
        return False if %invalid-suffixes{$suffix.chars};

        if is-valid-design(%towels, $max-towel-length, $suffix, %valid-suffixes, %invalid-suffixes) {
            %valid-suffixes{$suffix.chars}++;
            return True;
        } else {
            %invalid-suffixes{$suffix.chars}++;
        }
    }
    return False;
}

sub count-valid-design(%towels, $max-towel-length, $design, %suffixes) {
    my $count = 0;
    for min($max-towel-length, $design.chars)...1 {
        my $prefix = $design.substr(0,$_);
        next if %towels{$prefix}:!exists;
        if $_ == $design.chars {
            $count++;
            next;
        }

        my $suffix = $design.substr($_, *);
        if %suffixes{$suffix.chars}:!exists {
            %suffixes{$suffix.chars} = count-valid-design(%towels, $max-towel-length, $suffix, %suffixes);
        }
        $count += %suffixes{$suffix.chars};
    }
    return $count;
}