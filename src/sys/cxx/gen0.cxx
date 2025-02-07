#include "wrapper.hpp"
#include "autocxxgen_ffi.h"
#include <array>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <new>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

namespace {
template <typename T>
class impl;
} // namespace

class String;

#ifndef CXXBRIDGE1_RUST_STR
#define CXXBRIDGE1_RUST_STR
class Str final {
public:
  Str() noexcept;
  Str(const String &) noexcept;
  Str(const std::string &);
  Str(const char *);
  Str(const char *, std::size_t);

  Str &operator=(const Str &) &noexcept = default;

  explicit operator std::string() const;

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  Str(const Str &) noexcept = default;
  ~Str() noexcept = default;

  using iterator = const char *;
  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const Str &) const noexcept;
  bool operator!=(const Str &) const noexcept;
  bool operator<(const Str &) const noexcept;
  bool operator<=(const Str &) const noexcept;
  bool operator>(const Str &) const noexcept;
  bool operator>=(const Str &) const noexcept;

  void swap(Str &) noexcept;

private:
  class uninit;
  Str(uninit) noexcept;
  friend impl<Str>;

  std::array<std::uintptr_t, 2> repr;
};
#endif // CXXBRIDGE1_RUST_STR

#ifndef CXXBRIDGE1_IS_COMPLETE
#define CXXBRIDGE1_IS_COMPLETE
namespace detail {
namespace {
template <typename T, typename = std::size_t>
struct is_complete : std::false_type {};
template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};
} // namespace
} // namespace detail
#endif // CXXBRIDGE1_IS_COMPLETE

#ifndef CXXBRIDGE1_RELOCATABLE
#define CXXBRIDGE1_RELOCATABLE
namespace detail {
template <typename... Ts>
struct make_void {
  using type = void;
};

template <typename... Ts>
using void_t = typename make_void<Ts...>::type;

template <typename Void, template <typename...> class, typename...>
struct detect : std::false_type {};
template <template <typename...> class T, typename... A>
struct detect<void_t<T<A...>>, T, A...> : std::true_type {};

template <template <typename...> class T, typename... A>
using is_detected = detect<void, T, A...>;

template <typename T>
using detect_IsRelocatable = typename T::IsRelocatable;

template <typename T>
struct get_IsRelocatable
    : std::is_same<typename T::IsRelocatable, std::true_type> {};
} // namespace detail

template <typename T>
struct IsRelocatable
    : std::conditional<
          detail::is_detected<detail::detect_IsRelocatable, T>::value,
          detail::get_IsRelocatable<T>,
          std::integral_constant<
              bool, std::is_trivially_move_constructible<T>::value &&
                        std::is_trivially_destructible<T>::value>>::type {};
#endif // CXXBRIDGE1_RELOCATABLE

namespace detail {
template <typename T, typename = void *>
struct operator_new {
  void *operator()(::std::size_t sz) { return ::operator new(sz); }
};

template <typename T>
struct operator_new<T, decltype(T::operator new(sizeof(T)))> {
  void *operator()(::std::size_t sz) { return T::operator new(sz); }
};
} // namespace detail

template <typename T>
union MaybeUninit {
  T value;
  void *operator new(::std::size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};

namespace {
template <typename T>
void destroy(T *ptr) {
  ptr->~T();
}

template <bool> struct deleter_if {
  template <typename T> void operator()(T *) {}
};

template <> struct deleter_if<true> {
  template <typename T> void operator()(T *ptr) { ptr->~T(); }
};
} // namespace
} // namespace cxxbridge1
} // namespace rust

static_assert(
    ::rust::IsRelocatable<::c_uint>::value,
    "type c_uint should be trivially move constructible and trivially destructible in C++ to be used as an argument of `GetTimeDateString` or return value of `GetDayOfWeek`, `GetMonth`, `GetYear` in Rust");

extern "C" {
::std::string *cxxbridge1$autocxx_make_string_0xf17503e17c2ca26a(::rust::Str str_) noexcept {
  ::std::unique_ptr<::std::string> (*autocxx_make_string_0xf17503e17c2ca26a$)(::rust::Str) = ::autocxx_make_string_0xf17503e17c2ca26a;
  return autocxx_make_string_0xf17503e17c2ca26a$(str_).release();
}

::RE::Calendar *cxxbridge1$Calendar_autocxx_alloc_autocxx_wrapper_0xf17503e17c2ca26a() noexcept {
  ::RE::Calendar *(*Calendar_autocxx_alloc_autocxx_wrapper_0xf17503e17c2ca26a$)() = ::Calendar_autocxx_alloc_autocxx_wrapper_0xf17503e17c2ca26a;
  return Calendar_autocxx_alloc_autocxx_wrapper_0xf17503e17c2ca26a$();
}

void cxxbridge1$Calendar_autocxx_free_autocxx_wrapper_0xf17503e17c2ca26a(::RE::Calendar *arg0) noexcept {
  void (*Calendar_autocxx_free_autocxx_wrapper_0xf17503e17c2ca26a$)(::RE::Calendar *) = ::Calendar_autocxx_free_autocxx_wrapper_0xf17503e17c2ca26a;
  Calendar_autocxx_free_autocxx_wrapper_0xf17503e17c2ca26a$(arg0);
}

::RE::Calendar *cxxbridge1$RE_Calendar_GetSingleton_autocxx_wrapper_0xf17503e17c2ca26a() noexcept {
  ::RE::Calendar *(*RE_Calendar_GetSingleton_autocxx_wrapper_0xf17503e17c2ca26a$)() = ::RE_Calendar_GetSingleton_autocxx_wrapper_0xf17503e17c2ca26a;
  return RE_Calendar_GetSingleton_autocxx_wrapper_0xf17503e17c2ca26a$();
}
} // extern "C"

namespace RE {
extern "C" {
float RE$cxxbridge1$Calendar$GetCurrentGameTime(::RE::Calendar const &self) noexcept {
  float (::RE::Calendar::*GetCurrentGameTime$)() const = &::RE::Calendar::GetCurrentGameTime;
  return (self.*GetCurrentGameTime$)();
}

float RE$cxxbridge1$Calendar$GetDay(::RE::Calendar const &self) noexcept {
  float (::RE::Calendar::*GetDay$)() const = &::RE::Calendar::GetDay;
  return (self.*GetDay$)();
}
} // extern "C"
} // namespace RE

extern "C" {
::std::string *cxxbridge1$GetDayName_autocxx_wrapper_0xf17503e17c2ca26a(::RE::Calendar const &autocxx_gen_this) noexcept {
  ::std::unique_ptr<::std::string> (*GetDayName_autocxx_wrapper_0xf17503e17c2ca26a$)(::RE::Calendar const &) = ::GetDayName_autocxx_wrapper_0xf17503e17c2ca26a;
  return GetDayName_autocxx_wrapper_0xf17503e17c2ca26a$(autocxx_gen_this).release();
}
} // extern "C"

namespace RE {
extern "C" {
void RE$cxxbridge1$Calendar$GetDayOfWeek(::RE::Calendar const &self, ::c_uint *return$) noexcept {
  ::c_uint (::RE::Calendar::*GetDayOfWeek$)() const = &::RE::Calendar::GetDayOfWeek;
  new (return$) ::c_uint((self.*GetDayOfWeek$)());
}

float RE$cxxbridge1$Calendar$GetDaysPassed(::RE::Calendar const &self) noexcept {
  float (::RE::Calendar::*GetDaysPassed$)() const = &::RE::Calendar::GetDaysPassed;
  return (self.*GetDaysPassed$)();
}

void RE$cxxbridge1$Calendar$GetTimeDateString(::RE::Calendar const &self, char *a_dest, ::c_uint *a_max, bool a_showYear) noexcept {
  void (::RE::Calendar::*GetTimeDateString$)(char *, ::c_uint, bool) const = &::RE::Calendar::GetTimeDateString;
  (self.*GetTimeDateString$)(a_dest, ::std::move(*a_max), a_showYear);
}

float RE$cxxbridge1$Calendar$GetHour(::RE::Calendar const &self) noexcept {
  float (::RE::Calendar::*GetHour$)() const = &::RE::Calendar::GetHour;
  return (self.*GetHour$)();
}

float RE$cxxbridge1$Calendar$GetHoursPassed(::RE::Calendar const &self) noexcept {
  float (::RE::Calendar::*GetHoursPassed$)() const = &::RE::Calendar::GetHoursPassed;
  return (self.*GetHoursPassed$)();
}

void RE$cxxbridge1$Calendar$GetMonth(::RE::Calendar const &self, ::c_uint *return$) noexcept {
  ::c_uint (::RE::Calendar::*GetMonth$)() const = &::RE::Calendar::GetMonth;
  new (return$) ::c_uint((self.*GetMonth$)());
}
} // extern "C"
} // namespace RE

extern "C" {
::std::string *cxxbridge1$GetMonthName_autocxx_wrapper_0xf17503e17c2ca26a(::RE::Calendar const &autocxx_gen_this) noexcept {
  ::std::unique_ptr<::std::string> (*GetMonthName_autocxx_wrapper_0xf17503e17c2ca26a$)(::RE::Calendar const &) = ::GetMonthName_autocxx_wrapper_0xf17503e17c2ca26a;
  return GetMonthName_autocxx_wrapper_0xf17503e17c2ca26a$(autocxx_gen_this).release();
}

void cxxbridge1$GetTime_autocxx_wrapper_0xf17503e17c2ca26a(::RE::Calendar const &autocxx_gen_this, ::tm *placement_return_type) noexcept {
  void (*GetTime_autocxx_wrapper_0xf17503e17c2ca26a$)(::RE::Calendar const &, ::tm *) = ::GetTime_autocxx_wrapper_0xf17503e17c2ca26a;
  GetTime_autocxx_wrapper_0xf17503e17c2ca26a$(autocxx_gen_this, placement_return_type);
}
} // extern "C"

namespace RE {
extern "C" {
float RE$cxxbridge1$Calendar$GetTimescale(::RE::Calendar const &self) noexcept {
  float (::RE::Calendar::*GetTimescale$)() const = &::RE::Calendar::GetTimescale;
  return (self.*GetTimescale$)();
}

void RE$cxxbridge1$Calendar$GetYear(::RE::Calendar const &self, ::c_uint *return$) noexcept {
  ::c_uint (::RE::Calendar::*GetYear$)() const = &::RE::Calendar::GetYear;
  new (return$) ::c_uint((self.*GetYear$)());
}
} // extern "C"
} // namespace RE

extern "C" {
void cxxbridge1$Calendar_synthetic_destructor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a(::RE::Calendar *autocxx_gen_this) noexcept {
  void (*Calendar_synthetic_destructor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a$)(::RE::Calendar *) = ::Calendar_synthetic_destructor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a;
  Calendar_synthetic_destructor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a$(autocxx_gen_this);
}

::tm *cxxbridge1$tm_autocxx_alloc_autocxx_wrapper_0xf17503e17c2ca26a() noexcept {
  ::tm *(*tm_autocxx_alloc_autocxx_wrapper_0xf17503e17c2ca26a$)() = ::tm_autocxx_alloc_autocxx_wrapper_0xf17503e17c2ca26a;
  return tm_autocxx_alloc_autocxx_wrapper_0xf17503e17c2ca26a$();
}

void cxxbridge1$tm_autocxx_free_autocxx_wrapper_0xf17503e17c2ca26a(::tm *arg0) noexcept {
  void (*tm_autocxx_free_autocxx_wrapper_0xf17503e17c2ca26a$)(::tm *) = ::tm_autocxx_free_autocxx_wrapper_0xf17503e17c2ca26a;
  tm_autocxx_free_autocxx_wrapper_0xf17503e17c2ca26a$(arg0);
}

void cxxbridge1$tm_new_synthetic_move_ctor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a(::tm *autocxx_gen_this, ::tm *other) noexcept {
  void (*tm_new_synthetic_move_ctor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a$)(::tm *, ::tm *) = ::tm_new_synthetic_move_ctor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a;
  tm_new_synthetic_move_ctor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a$(autocxx_gen_this, other);
}

void cxxbridge1$tm_new_synthetic_const_copy_ctor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a(::tm *autocxx_gen_this, ::tm const &other) noexcept {
  void (*tm_new_synthetic_const_copy_ctor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a$)(::tm *, ::tm const &) = ::tm_new_synthetic_const_copy_ctor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a;
  tm_new_synthetic_const_copy_ctor_0xf17503e17c2ca26a_autocxx_wrapper_0xf17503e17c2ca26a$(autocxx_gen_this, other);
}

static_assert(::rust::detail::is_complete<::RE::Calendar>::value, "definition of Calendar is required");
static_assert(sizeof(::std::unique_ptr<::RE::Calendar>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::RE::Calendar>) == alignof(void *), "");
void cxxbridge1$unique_ptr$RE$Calendar$null(::std::unique_ptr<::RE::Calendar> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::RE::Calendar>();
}
::RE::Calendar *cxxbridge1$unique_ptr$RE$Calendar$uninit(::std::unique_ptr<::RE::Calendar> *ptr) noexcept {
  ::RE::Calendar *uninit = reinterpret_cast<::RE::Calendar *>(new ::rust::MaybeUninit<::RE::Calendar>);
  ::new (ptr) ::std::unique_ptr<::RE::Calendar>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$RE$Calendar$raw(::std::unique_ptr<::RE::Calendar> *ptr, ::RE::Calendar *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::RE::Calendar>(raw);
}
::RE::Calendar const *cxxbridge1$unique_ptr$RE$Calendar$get(::std::unique_ptr<::RE::Calendar> const &ptr) noexcept {
  return ptr.get();
}
::RE::Calendar *cxxbridge1$unique_ptr$RE$Calendar$release(::std::unique_ptr<::RE::Calendar> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$RE$Calendar$drop(::std::unique_ptr<::RE::Calendar> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::RE::Calendar>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::RE::Calendar>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::RE::Calendar>) == alignof(void *), "");
void cxxbridge1$shared_ptr$RE$Calendar$null(::std::shared_ptr<::RE::Calendar> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::RE::Calendar>();
}
::RE::Calendar *cxxbridge1$shared_ptr$RE$Calendar$uninit(::std::shared_ptr<::RE::Calendar> *ptr) noexcept {
  ::RE::Calendar *uninit = reinterpret_cast<::RE::Calendar *>(new ::rust::MaybeUninit<::RE::Calendar>);
  ::new (ptr) ::std::shared_ptr<::RE::Calendar>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$RE$Calendar$clone(::std::shared_ptr<::RE::Calendar> const &self, ::std::shared_ptr<::RE::Calendar> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::RE::Calendar>(self);
}
::RE::Calendar const *cxxbridge1$shared_ptr$RE$Calendar$get(::std::shared_ptr<::RE::Calendar> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$RE$Calendar$drop(::std::shared_ptr<::RE::Calendar> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::RE::Calendar>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::RE::Calendar>) == alignof(void *), "");
void cxxbridge1$weak_ptr$RE$Calendar$null(::std::weak_ptr<::RE::Calendar> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::RE::Calendar>();
}
void cxxbridge1$weak_ptr$RE$Calendar$clone(::std::weak_ptr<::RE::Calendar> const &self, ::std::weak_ptr<::RE::Calendar> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::RE::Calendar>(self);
}
void cxxbridge1$weak_ptr$RE$Calendar$downgrade(::std::shared_ptr<::RE::Calendar> const &shared, ::std::weak_ptr<::RE::Calendar> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::RE::Calendar>(shared);
}
void cxxbridge1$weak_ptr$RE$Calendar$upgrade(::std::weak_ptr<::RE::Calendar> const &weak, ::std::shared_ptr<::RE::Calendar> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::RE::Calendar>(weak.lock());
}
void cxxbridge1$weak_ptr$RE$Calendar$drop(::std::weak_ptr<::RE::Calendar> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::tm>::value, "definition of tm is required");
static_assert(sizeof(::std::unique_ptr<::tm>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::tm>) == alignof(void *), "");
void cxxbridge1$unique_ptr$tm$null(::std::unique_ptr<::tm> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::tm>();
}
::tm *cxxbridge1$unique_ptr$tm$uninit(::std::unique_ptr<::tm> *ptr) noexcept {
  ::tm *uninit = reinterpret_cast<::tm *>(new ::rust::MaybeUninit<::tm>);
  ::new (ptr) ::std::unique_ptr<::tm>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$tm$raw(::std::unique_ptr<::tm> *ptr, ::tm *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::tm>(raw);
}
::tm const *cxxbridge1$unique_ptr$tm$get(::std::unique_ptr<::tm> const &ptr) noexcept {
  return ptr.get();
}
::tm *cxxbridge1$unique_ptr$tm$release(::std::unique_ptr<::tm> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$tm$drop(::std::unique_ptr<::tm> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::tm>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::tm>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::tm>) == alignof(void *), "");
void cxxbridge1$shared_ptr$tm$null(::std::shared_ptr<::tm> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::tm>();
}
::tm *cxxbridge1$shared_ptr$tm$uninit(::std::shared_ptr<::tm> *ptr) noexcept {
  ::tm *uninit = reinterpret_cast<::tm *>(new ::rust::MaybeUninit<::tm>);
  ::new (ptr) ::std::shared_ptr<::tm>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$tm$clone(::std::shared_ptr<::tm> const &self, ::std::shared_ptr<::tm> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::tm>(self);
}
::tm const *cxxbridge1$shared_ptr$tm$get(::std::shared_ptr<::tm> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$tm$drop(::std::shared_ptr<::tm> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::tm>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::tm>) == alignof(void *), "");
void cxxbridge1$weak_ptr$tm$null(::std::weak_ptr<::tm> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::tm>();
}
void cxxbridge1$weak_ptr$tm$clone(::std::weak_ptr<::tm> const &self, ::std::weak_ptr<::tm> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::tm>(self);
}
void cxxbridge1$weak_ptr$tm$downgrade(::std::shared_ptr<::tm> const &shared, ::std::weak_ptr<::tm> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::tm>(shared);
}
void cxxbridge1$weak_ptr$tm$upgrade(::std::weak_ptr<::tm> const &weak, ::std::shared_ptr<::tm> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::tm>(weak.lock());
}
void cxxbridge1$weak_ptr$tm$drop(::std::weak_ptr<::tm> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::tm> *cxxbridge1$std$vector$tm$new() noexcept {
  return new ::std::vector<::tm>();
}
::std::size_t cxxbridge1$std$vector$tm$size(::std::vector<::tm> const &s) noexcept {
  return s.size();
}
::tm *cxxbridge1$std$vector$tm$get_unchecked(::std::vector<::tm> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$tm$push_back(::std::vector<::tm> *v, ::tm *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$tm$pop_back(::std::vector<::tm> *v, ::tm *out) noexcept {
  ::new (out) ::tm(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::tm>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::tm>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$tm$null(::std::unique_ptr<::std::vector<::tm>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::tm>>();
}
void cxxbridge1$unique_ptr$std$vector$tm$raw(::std::unique_ptr<::std::vector<::tm>> *ptr, ::std::vector<::tm> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::tm>>(raw);
}
::std::vector<::tm> const *cxxbridge1$unique_ptr$std$vector$tm$get(::std::unique_ptr<::std::vector<::tm>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::tm> *cxxbridge1$unique_ptr$std$vector$tm$release(::std::unique_ptr<::std::vector<::tm>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$tm$drop(::std::unique_ptr<::std::vector<::tm>> *ptr) noexcept {
  ptr->~unique_ptr();
}
} // extern "C"
