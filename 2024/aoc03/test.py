import re

a = r"\[[^\]]*(A)[^\]]*\]"

b = "sdfefAfef[AsdfeAfefs]fsdA"

start = re.search(r"^.*\]", b)
if start is not None:
    start = start.group(0)
    b = b.replace(start, "")

end = re.search(r"\[[^\]]*$", b)
if end is not None:
    end = end.group(0)
    b = b.replace(end, "")

middles = []

while True:
    try:
        middles.append(re.search(r"\[.*\]", b).group(0))
        b = b.replace(middles[-1], "")
    except:
        break

print(b)
