from subprocess import call

call(["cargo", "fmt"])
call(["cargo", "update"])
call(["cargo", "run", "--release"])
