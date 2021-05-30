$Test = {
    Name => "Unpack (SIMPLE)",
    Command => "-f test/available/test.bpx unpack",
    Description => "Test the unpack command",
    Status => 0
};

sub TestBegin {

}

sub TestEnd {
    my $res = EnsureEqual("LICENSE_LF.txt", "test/LICENSE_LF.txt");
    unlink("LICENSE_LF.txt");
    unlink("test/LICENSE_LF.txt");
    return $res;
}
