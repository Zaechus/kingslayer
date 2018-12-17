from subprocess import call

call(["cargo", "fmt"])

call(["cargo", "clippy"])

call(["cargo", "check"])

call(["cargo", "test"])

call(["cargo", "doc"])
call(["cargo", "build"])
