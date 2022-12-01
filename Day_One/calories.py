with open("calories.txt", "r") as fw:
    print(max([sum(int(calsingle) for calsingle in calgroup.split("\n")) for calgroup in fw.read().split("\n\n")]))

