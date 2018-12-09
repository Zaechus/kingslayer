import os
from subprocess import call

os.chdir("site")
call(["npm", "run", "serve"])
os.chdir("..")