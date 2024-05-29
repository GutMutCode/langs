a = [1, 2, 3, 4]
textfile = open("text.txt", "w")
show(textfile, a)
# println(textfile, a)
close(textfile)

# open file in scope - automatically closes
open("text.txt", "w") do textfile
  println(textfile, a)
end
