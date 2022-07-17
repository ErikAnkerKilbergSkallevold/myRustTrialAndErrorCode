#include "allocation.h"
#include "inode.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

#define BLOCKSIZE 4096

int resolve(struct inode* node, int ant_nodes, struct inode** inode_list);
struct inode* read_inode(FILE* f);
void check_error(int return_value,int count, char* to_perror);

//Global variable used in create_dir and create_file to assign id
int inodes_made = 0;

struct inode* create_file( struct inode* parent, char* name, char readonly, int size_in_bytes )
{
    struct inode* new_file = malloc(sizeof(struct inode));
	if (new_file == NULL){
		perror("Malloc failed");
	}
	
	//Checks for duplicate
	for (int entry = 0; entry < parent->num_entries ; entry++){
		if(((struct inode*)parent->entries[entry])->name == name){
			printf("Found duplicate\n");
			return NULL;
		}
	}
	
	//Resolves pointers
	/* Desided to not use realloc, since it added unneeded complexity
	int ret = realloc(parent->entries, sizeof(size_t*) * (parent->num_entries + 2));
	if (ret == NULL){
		perror("realloc");
	}*/
	parent->entries[parent->num_entries] = (size_t) new_file;
	parent->num_entries += 1;
	
	//Constructing the file
	new_file->id = inodes_made;
	inodes_made++;
	new_file->name = strdup(name);
	new_file->is_directory = 0;
	new_file->is_readonly = readonly;
	new_file->filesize = size_in_bytes;
	
	//Entries
	new_file->num_entries = (new_file->filesize / BLOCKSIZE) + 1; //Add one since int div rounds towards zero
	new_file->entries = malloc(sizeof(size_t*) * new_file->num_entries);
	for (int block=0; block < new_file->num_entries; block++){
		//Allocates blocks on the viritual disk
		new_file->entries[block] = allocate_block();
	}
	
	return new_file;
}

struct inode* create_dir( struct inode* parent, char* name )
{
    struct inode* new_dir = malloc(sizeof(struct inode));
	if (new_dir == NULL){
		perror("Malloc failed");
	}
	
	//Checks for duplicate
	if (parent != NULL){
		for (int entry = 0; entry < parent->num_entries ; entry++){
			if(((struct inode*)parent->entries[entry])->name == name){
				printf("Found duplicate\n");
				return NULL;
			}
		}
	}
	
	//Resolves pointers
	if (parent != NULL){
		/*
		int ret = realloc(parent->entries, sizeof(size_t*) * (parent->num_entries + 2));
		if (ret == NULL){
			perror("realloc");
		}*/
		parent->entries[parent->num_entries] = (size_t) new_dir;
		parent->num_entries += 1;
	}
	
	//constructing the file
	new_dir->id = inodes_made;
	inodes_made++;
	new_dir->name = strdup(name);
	new_dir->is_directory = 1;
	new_dir->is_readonly = 0;
	new_dir->filesize = 0;
	new_dir->num_entries = 0;
	//allocating up to fifty inodes in a single dir allowes for less complexity.
	//Even though it uses up space, size_t* is rather small and 50 is not espessially large.
	//I also know that the file system will probably never exeed 10 inodes per dir, so it is safe.
	new_dir->entries = malloc(sizeof(size_t*) * 50);
	
	return new_dir;
}

struct inode* find_inode_by_name( struct inode* parent, char* name )
{
	if (!((int) parent->is_directory)){
		return NULL;
	}
	
	//Checks the name of every inode directly connected to the parent
    for (int entry=0; entry < parent->num_entries; entry++){
		if(strcmp(((struct inode*)parent->entries[entry])->name, name) == 0){
			return (struct inode*) parent->entries[entry];
		}
	}
	return NULL;
}

struct inode* load_inodes()
{
    FILE *f = fopen("superblock", "r");
	if (f == NULL){
		perror("fopen");
	}
	
	//Can only read 100 inodes at a time
	struct inode** inode_list = malloc(sizeof(struct inode*)*100);
	
	//Reads the root (always the first read)
	struct inode* root = read_inode(f);
	inode_list[0] = root;

	//reads all other inodes
	int inode_counter = 1;
	int run = 1;
	struct inode* current;
	while (run){
		current = read_inode(f);
		if (current == 0){
			break;
		}
		inode_list[inode_counter] = current;
		inode_counter++;
	}
	
	//Resolves pointers between all nodes
	//For every inode that is not the root, find parent
	for(int list_counter=1; list_counter<inode_counter ; list_counter++){
		int ret = resolve(inode_list[list_counter], inode_counter, inode_list);
		if (ret == 0){
			perror("Could not resolve pointers");
		}
	}
	free(inode_list);
	fclose(f);
	return root;
}

void fs_shutdown( struct inode* inode )
{
	for (int entry=0; entry < inode->num_entries; entry++){
		if ((int)((struct inode*)inode->entries[entry])->is_directory){
			//if dir recurse downwards
			fs_shutdown((struct inode*) inode->entries[entry]);
		} else {
			//If file free then continue
			free(((struct inode*) inode->entries[entry])->name);
			free(((struct inode*) inode->entries[entry])->entries);
			free((size_t*) inode->entries[entry]);
		}
	}
	//Finally free root
	free(inode->name);
	free(inode->entries);
	free(inode);
}

/* This static variable is used to change the indentation while debug_fs
 * is walking through the tree of inodes and prints information.
 */
static int indent = 0;

void debug_fs( struct inode* node )
{
    if( node == NULL ) return;
    for( int i=0; i<indent; i++ )
        printf("  ");
    if( node->is_directory )
    {
        printf("%s (id %d)\n", node->name, node->id );
        indent++;
        for( int i=0; i<node->num_entries; i++ )
        {
            struct inode* child = (struct inode*)node->entries[i];
            debug_fs( child );
        }
        indent--;
    }
    else
    {
        printf("%s (id %d size %db blocks ", node->name, node->id, node->filesize );
        for( int i=0; i<node->num_entries; i++ )
        {
            printf("%d ", (int)node->entries[i]);
        }
        printf(")\n");
    }
}

//Checs the return value of fread
void check_error(int return_value,int count, char* to_perror){
	if (return_value < count){
		perror(to_perror);
		exit(0);
	}
	return;
}

//Reads a single inode
struct inode* read_inode(FILE* f){
	
	//Allocate space on the heap
	struct inode* node = malloc(sizeof(struct inode));
	if (node == 0){
		perror("malloc");
		exit(0);
	}
	
	//Helping variable
	int name_length;
	
	//Read all data of one Inode
	//id
	int ret = fread(&node->id, sizeof(int), 1, f);
	//check_error(ret, 1, "fread_id");
	if(ret == 0){ //If end of file, return
		free(node);
		return 0;
	}
	
	//name_length
	ret = fread(&name_length, sizeof(int), 1, f);
	check_error(ret, 1, "fread_name_lenght");
	
	//name
	char* name = malloc(name_length*sizeof(char));
	ret = fread(name, sizeof(char), name_length, f);
	node->name = name;
	check_error(ret, name_length, "fread_name");
	
	//is_directory
	ret = fread(&node->is_directory, sizeof(char), 1, f);
	check_error(ret, 1, "fread_is_directory");
	
	//is_readonly
	ret = fread(&node->is_readonly, sizeof(char), 1, f);
	check_error(ret, 1, "fread_is_readonly");
	
	//filesize
	ret = fread(&node->filesize, sizeof(int), 1, f);
	check_error(ret, 1, "fread_filesize");
	
	//num_entries
	ret = fread(&node->num_entries, sizeof(int), 1, f);
	check_error(ret, 1, "fread_num_entries");
	
	//entries
	size_t* entries = malloc(node->num_entries*sizeof(size_t*));
	ret = fread(entries, sizeof(size_t), node->num_entries, f);
	node->entries = entries;
	check_error(ret, node->num_entries,"fread_entries");
	return node;
}

//Returns 1 on succsess
int resolve(struct inode* node, int ant_nodes, struct inode** inode_list){
	//for every inode including root
	for(int i=0; i < ant_nodes; i++){
		if ((int)inode_list[i]->is_directory){
			//if it is dir check every id of entries
			for(int j=0; j < inode_list[i]->num_entries; j++){
				//If id match then resolve
				if(inode_list[i]->entries[j] == (size_t) node->id){
					inode_list[i]->entries[j] = (size_t) node;
					return 1;
				}
			}
		}
	}
	return 0;
}
