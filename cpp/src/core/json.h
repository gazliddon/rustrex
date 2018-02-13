#ifndef JSON_H_KJPYUNDB
#define JSON_H_KJPYUNDB

#include <nlohmann/json.hpp>

#include "structs.h"

void to_json(nlohmann::json & j, regs_t const & _r) ;
void to_json(nlohmann::json & j, mem_descriptor_t const & _mem) ;
void to_json(nlohmann::json & j, cpu_state_t const & _s) ;
void to_json(nlohmann::json & j, run_log_t const & _r) ;


void from_json(nlohmann::json const & j, regs_t & _r) ;
void from_json(nlohmann::json const & j, mem_descriptor_t & _mem) ;
void from_json(nlohmann::json const & j, cpu_state_t & _s) ;
void from_json(nlohmann::json const & j, run_log_t & _r) ;

#endif /* end of include guard: JSON_H_KJPYUNDB */
