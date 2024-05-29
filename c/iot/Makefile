CC = gcc
CFLAGS = -Wall -Werror
TARGET = a.out
OBJS = main.o foo.o bar.o
# LIBS = 
# LDFLAGS =
# LDLIBS =

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) -o $@ $(OBJS)

main.o: foo.h bar.h main.c
foo.o:	foo.h foo.c
bar.o:	bar.h bar.c

clean :
	rm -f $(TARGET) $(OBJS)
