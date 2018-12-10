import os, re, shutil
from subprocess import call

shutil.rmtree("docs", ignore_errors=True)

os.chdir("site")
call(["npm", "run", "build"])
os.chdir("..")

shutil.copytree("site/dist", "docs")
shutil.rmtree("site/dist")

data = ""
with open("docs/index.html", "r") as index:
    data = index.read().replace("\n", "")
    data = re.sub(r"\b=/\b", "=", data)

with open("docs/index.html", "w") as index:
    index.write(data)