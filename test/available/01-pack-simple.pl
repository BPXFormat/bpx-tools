$Test = {
    Name => "Pack (SIMPLE)",
    Command => "-f test.bpx pack test/LICENSE_LF.txt",
    Description => "Test the pack command",
    Status => 0
};

sub TestBegin {
    CRLFToLF("test/LICENSE.txt", "test/LICENSE_LF.txt");
}

sub TestEnd {
    return EnsureEqual("test.bpx", "test/available/test.bpx");
}
