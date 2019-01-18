from subprocess import call

call(["cargo", "fmt"])
call(["cargo", "update"])

call(["cargo", "clippy"])
call(["cargo", "c"])

call(["cargo", "doc"])
call(["cargo", "build"])

call(["cargo", "bench"])
call(["cargo", "test"])
