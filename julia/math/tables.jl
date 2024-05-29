# With PrettyTables, you can customise the table as you wish
using Pkg;
Pkg.add("PrettyTables");
using PrettyTables

# create random array of 4(rows) x 4(colums)
M = rand(4, 4)
# println(M)

# display table - convert a matrix into a table
# pretty_table(M)

# backends
# - text (default, outputting the table in raw text)
# - html (outputs HTML defining the table)
# - latex (outputs LaTeX defining the table as a `tabular` object)

# PrettyTables also allows creating preset configurations which can be used for multiple tables.
tableconfig = set_pt_conf(
  tf=tf_markdown, # table format : markdown
  columns_width=15,
  alignment=:c, # center
  formatters=((value, i, j) -> round(value, digits=3)) # convert floats
);

# pretty_table_with_conf(tableconfig, M; header=["Player $i" for i = 1:4])

# Using macros
@ptconf(
  backend = Val(:latex),
  highlighters = LatexHighlighter((data, i, j) -> data[i, j] > 0.5, "textbf"),
  wrap_table = false,
  alignment = :c
)
@pt :header = ["Player $i" for i = 1:4] M
@ptconfclean

# write to file
open("tablefile.txt", "w") do io
  pretty_table(io, M; header=["Player $i" for i = 1:4])
end
