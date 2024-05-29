using Pkg;
Pkg.add("Plots");
using Plots

xdata = collect(1:10)
ydata = [10, 6, 5, 4, 8, 9, 3, 4, 2, 1]
plot(xdata, ydata)

plot(sinc)

# config the appearance of the plot
# title - controls the title of the plot
# label - controls the name given to the line in the legend, `legend` controls whether the legend appears or not
# xlims, ylims, zlims - control the extent of the corresponding axis
# grid - controls whether a grid appears in the background of the plot, while `minorgrid` does the same with a grid with smaller divisions
# showaxis - controls which axes are shown, while `ticks` controls the numbering of the axes
plot(
  tan,
  title="The tangent function",
  legend=false,
  xlims=(-π, π),
  ylims=(-10, 10),
  grid=false,
  showaxis=:x,
  framestyle=:origin
)

# seriestype - controls the type of the plot
# `StatsPlots` - additional methods of statistical data visualization
# `GraphRecipes` - plotting graphs of the graph theoretical kind
plot(
  [(0, 1), (4, 3), (1, 6), (5, 2), (6, 3), (2, 4)],
  label="Blue",
  seriestype=:scatter,
  xlims=(-1, 7),
  ylims=(-1, 7),
  framestyle=:origin
)

# plots can be saved as variables
barchart = bar(
  [("A", 4), ("B", 5), ("C", 1), ("D", 4), ("E", 6), ("F", 2)],
  legend=false,
  fillcolor=:green
)

# and added to a plot
bar!(
  barchart,
  [("A", 2), ("B", 6), ("C", 3), ("D", 3), ("E", 6), ("F", 5)],
  color=:red,
  bar_width=0.5
)

barchart = bar()

function customfnplot(fnname)
  return plot(
    eval(Meta.parse(fnname)), # parse a string into an expression before compiling
    xlims=(0, 4π),
    legend=false,
    title=fnname
  )
end

sinplot = customfnplot("sin")
cosplot = customfnplot("cos")

doubleplot = plot(sinplot, cosplot, layout=(2, 1))

# Plots can be saved as files by the function `savefig`, or can be saved by file type specific functions such as `png`
# The following code, if run, would save the `doubleplot` graph as "sinandcos.png"
png(doubleplot, "sinandcos")
