#ifndef FILES_H_YWJDOMM4
#define FILES_H_YWJDOMM4

#include <vector>
#include <fstream>

#include "mem.h"

size_t get_file_size(char const* _fileName) ;
std::vector<uint8_t> load_file(char const* _name) ;
void load_file(char const* _fileName, cMemIO& _mem, uint16_t _addr) ;


#endif /* end of include guard: FILES_H_YWJDOMM4 */
