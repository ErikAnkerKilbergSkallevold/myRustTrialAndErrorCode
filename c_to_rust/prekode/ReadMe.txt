How to run:
1. Ensure filestructure:
.
├── allocation.c
├── allocation.h
├── block_allocation_table
├── create_example1
│   ├── block_allocation_table
│   ├── create_fs.c
├── create_example2
│   ├── block_allocation_table
│   ├── create_fs.c
├── create_example3
│   ├── block_allocation_table
│   ├── create_fs.c
├── inode.c
├── inode.h
├── load_example1
│   ├── block_allocation_table
│   ├── load_fs.c
│   └── superblock
├── load_example2
│   ├── block_allocation_table
│   ├── load_fs.c
│   └── superblock
├── load_example3
│   ├── block_allocation_table
│   ├── load_fs.c
│   └── superblock
└── Makefile

2. to compile everything: make all
3. to run w/o valgrind: make run_(create/load)(1/2/3)
4. to run w valgrind: make valgrind_(create/load)(1/2/3)
5. to run load 1, 2 and 3: make run_all_load
6. to run create 1, 2 and 3: make run_all_create
7. to run both load 1, 2 and 3 and create 1, 2 and 3: make run_all

Design document:

load_inodes:
Load_inodes use a help function called read inode that reads a single inode from the given filepointer.
Since the filpointer keeps track of where in the file fread reads from, File* can be sent as an
argument to a function.

I use a doublepointer list to keep track of all the files read in. I also have a helping function that
finds the parent to an inode that is called resolve. I call this function for every inode in the
doublipointed list except from the root.
The resolve function loops through every node including the root and every entry in every node.
For every entry it checks if the id matches and if it does it resolves the pointers.

create_dir:
Because I decided to not use realloc, I allocate space for 50 entries in each dir.
This of course costs memory, but simplifies the pointer resolving considebly and is therefore
a worthy tradeof in my opinion. If over 50 inodes were to be added to the dir there is however no overflow
potection and it would probably chrash. The viritul disk we use would be full under normal use long before
a single dir got over 50 entries, so this is not a huge issue.

create_file:
create_file is very simmilar to create_dir, but insted of allocating space for other inodes I use the
allocate_block function.

find_file_by_name:
It consists of a simple for loop running through all the entries the parent node holds. If I were to
upgrade it to search thrugh everything connected (not directly connected) to the parent I would just
call the function recursivly.

fs_shutdown:
I solved this recursivly, calling the function for every dir connected to the previus.
This works fantasticly, the only problem I can think of is excecivly large filesystems that can cause
a stack overflow, but this will very rarly be the case.
