from subprocess import call

call(["cargo", "fmt"])

call(["cargo", "update"])

call(["cargo", "clippy"])

call(["cargo", "check"])

call(["cargo", "test"])
call(["cargo", "bench"])

call(["cargo", "doc"])
call(["cargo", "build"])
