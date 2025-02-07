#ifndef __AUTOCXXGEN_H__
#define __AUTOCXXGEN_H__

#include <memory>
#include <string>
#include "cxx.h"
#include <stddef.h>
#ifndef AUTOCXX_NEW_AND_DELETE_PRELUDE
#define AUTOCXX_NEW_AND_DELETE_PRELUDE
// Mechanics to call custom operator new and delete
template <typename T>
auto delete_imp(T *ptr, int) -> decltype((void)T::operator delete(ptr)) {
  T::operator delete(ptr);
}
template <typename T> void delete_imp(T *ptr, long) { ::operator delete(ptr); }
template <typename T> void delete_appropriately(T *obj) {
  // 0 is a better match for the first 'delete_imp' so will match
  // preferentially.
  delete_imp(obj, 0);
}
template <typename T>
auto new_imp(size_t count, int) -> decltype(T::operator new(count)) {
  return T::operator new(count);
}
template <typename T> void *new_imp(size_t count, long) {
  return ::operator new(count);
}
template <typename T> T *new_appropriately() {
  // 0 is a better match for the first 'delete_imp' so will match
  // preferentially.
  return static_cast<T *>(new_imp<T>(sizeof(T), 0));
}
#endif // AUTOCXX_NEW_AND_DELETE_PRELUDE
#include "wrapper.hpp"

typedef unsigned int c_uint;

inline std::unique_ptr<std::string> autocxx_make_string_0xf17503e17c2ca26a(::rust::Str str) { return std::make_unique<std::string>(std::string(str)); }
inline RE::Calendar* Calendar_autocxx_alloc_autocxx_wrapper_0xf17503e17c2ca26a()  { return new_appropriately<RE::Calendar>();; }
inline void Calendar_autocxx_free_autocxx_wrapper_0xf17503e17c2ca26a(RE::Calendar* arg0)  { delete_appropriately<RE::Calendar>(arg0);; }
inline RE::Calendar* RE_Calendar_GetSingleton_autocxx_wrapper_0xf17503e17c2ca26a()  { return RE::Calendar::GetSingleton(); }
inline std::unique_ptr<std::string> GetDayName_autocxx_wrapper_0xf17503e17c2ca26a(const RE::Calendar& autocxx_gen_this)  { return std::make_unique<std::string>(autocxx_gen_this.GetDayName()); }
inline std::unique_ptr<std::string> GetMonthName_autocxx_wrapper_0xf17503e17c2ca26a(const RE::Calendar& autocxx_gen_this)  { return std::make_unique<std::string>(autocxx_gen_this.GetMonthName()); }
inline void GetTime_autocxx_wrapper_0xf17503e17c2ca26a(const RE::Calendar& autocxx_gen_this, tm* arg1)  { new(arg1) tm(autocxx_gen_this.GetTime()); }
inline void Calendar_synthetic_destructor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a(RE::Calendar* arg0)  { { using RE::Calendar; arg0->Calendar::~Calendar(); }; }
inline tm* tm_autocxx_alloc_autocxx_wrapper_0xf17503e17c2ca26a()  { return new_appropriately<tm>();; }
inline void tm_autocxx_free_autocxx_wrapper_0xf17503e17c2ca26a(tm* arg0)  { delete_appropriately<tm>(arg0);; }
inline void tm_new_synthetic_move_ctor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a(tm* autocxx_gen_this, tm* arg1)  { new (autocxx_gen_this) tm(std::move(*arg1)); }
inline void tm_new_synthetic_const_copy_ctor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a(tm* autocxx_gen_this, const tm& arg1)  { new (autocxx_gen_this) tm(arg1); }
#endif // __AUTOCXXGEN_H__
