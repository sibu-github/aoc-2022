#!/usr/bin/perl

use diagnostics;
use strict;

my $input_file = "inputs.txt";
my $highest_1 = 0;
my $highest_2 = 0;
my $highest_3 = 0;
my $sum = 0;

open(FH, '<', $input_file) or die $!;
while(<FH>){
	my $line = $_;
	if($line eq "\n"){
		if($sum > $highest_1){
			$highest_3 = $highest_2;
			$highest_2 = $highest_1;
			$highest_1 = $sum;
		} elsif($sum > $highest_2){
			$highest_3 = $highest_2;
			$highest_2 = $sum;
		} elsif($sum > $highest_3){
			$highest_3 = $sum;
		}
		$sum = 0;
	} else {
		my $num = int($line);
		$sum = $sum + $num;
	}
}

if($sum > $highest_1){
	$highest_3 = $highest_2;
	$highest_2 = $highest_1;
	$highest_1 = $sum;
} elsif($sum > $highest_2){
	$highest_3 = $highest_2;
	$highest_2 = $sum;
} elsif($sum > $highest_3){
	$highest_3 = $sum;
}

print "\n";
print("Highest:", $highest_1);
print "\n";
print("Total of highest 3:", $highest_1 + $highest_2 + $highest_3);
print "\n";

close(FH);