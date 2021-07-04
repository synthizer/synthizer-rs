#define PLF_COLONY_TEST_DEBUG

#if defined(_MSC_VER)
	#define PLF_FORCE_INLINE __forceinline

	#if _MSC_VER < 1600
		#define PLF_NOEXCEPT throw()
		#define PLF_NOEXCEPT_SWAP(the_allocator)
		#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator) throw()
	#elif _MSC_VER == 1600
		#define PLF_MOVE_SEMANTICS_SUPPORT
		#define PLF_NOEXCEPT throw()
		#define PLF_NOEXCEPT_SWAP(the_allocator)
		#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator) throw()
	#elif _MSC_VER == 1700
		#define PLF_TYPE_TRAITS_SUPPORT
		#define PLF_ALLOCATOR_TRAITS_SUPPORT
		#define PLF_MOVE_SEMANTICS_SUPPORT
		#define PLF_NOEXCEPT throw()
		#define PLF_NOEXCEPT_SWAP(the_allocator)
		#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator) throw()
	#elif _MSC_VER == 1800
		#define PLF_TYPE_TRAITS_SUPPORT
		#define PLF_ALLOCATOR_TRAITS_SUPPORT
		#define PLF_VARIADICS_SUPPORT // Variadics, in this context, means both variadic templates and variadic macros are supported
		#define PLF_MOVE_SEMANTICS_SUPPORT
		#define PLF_NOEXCEPT throw()
		#define PLF_NOEXCEPT_SWAP(the_allocator)
		#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator) throw()
		#define PLF_INITIALIZER_LIST_SUPPORT
	#elif _MSC_VER >= 1900
		#define PLF_ALIGNMENT_SUPPORT
		#define PLF_TYPE_TRAITS_SUPPORT
		#define PLF_ALLOCATOR_TRAITS_SUPPORT
		#define PLF_VARIADICS_SUPPORT
		#define PLF_MOVE_SEMANTICS_SUPPORT
		#define PLF_NOEXCEPT noexcept
		#define PLF_NOEXCEPT_SWAP(the_allocator) noexcept(std::allocator_traits<the_allocator>::propagate_on_container_swap::value || std::allocator_traits<the_allocator>::is_always_equal::value)
		#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator) noexcept(std::allocator_traits<the_allocator>::propagate_on_container_move_assignment::value || std::allocator_traits<the_allocator>::is_always_equal::value)
		#define PLF_INITIALIZER_LIST_SUPPORT
	#endif

	#if defined(_MSVC_LANG) && (_MSVC_LANG >= 201703L)
		#define PLF_CONSTEXPR constexpr
		#define PLF_CONSTEXPR_SUPPORT
	#else
		#define PLF_CONSTEXPR
	#endif

	#if defined(_MSVC_LANG) && (_MSVC_LANG > 201703L)
		#define PLF_CPP20_SUPPORT
	#endif

#elif defined(__cplusplus) && __cplusplus >= 201103L // C++11 support, at least
	#define PLF_FORCE_INLINE // note: GCC creates faster code without forcing inline

	#if defined(__GNUC__) && defined(__GNUC_MINOR__) && !defined(__clang__) // If compiler is GCC/G++
		#if (__GNUC__ == 4 && __GNUC_MINOR__ >= 3) || __GNUC__ > 4 // 4.2 and below do not support variadic templates
			#define PLF_VARIADICS_SUPPORT
		#endif

		#if (__GNUC__ == 4 && __GNUC_MINOR__ >= 4) || __GNUC__ > 4 // 4.3 and below do not support initializer lists
			#define PLF_INITIALIZER_LIST_SUPPORT
		#endif

		#if (__GNUC__ == 4 && __GNUC_MINOR__ < 6) || __GNUC__ < 4
			#define PLF_NOEXCEPT throw()
			#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator)
			#define PLF_NOEXCEPT_SWAP(the_allocator)
		#elif __GNUC__ < 6
			#define PLF_NOEXCEPT noexcept
			#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator) noexcept
			#define PLF_NOEXCEPT_SWAP(the_allocator) noexcept
		#else // C++17 support
			#define PLF_NOEXCEPT noexcept
			#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator) noexcept(std::allocator_traits<the_allocator>::propagate_on_container_move_assignment::value || std::allocator_traits<the_allocator>::is_always_equal::value)
			#define PLF_NOEXCEPT_SWAP(the_allocator) noexcept(std::allocator_traits<the_allocator>::propagate_on_container_swap::value || std::allocator_traits<the_allocator>::is_always_equal::value)
		#endif

		#if (__GNUC__ == 4 && __GNUC_MINOR__ >= 7) || __GNUC__ > 4
			#define PLF_ALLOCATOR_TRAITS_SUPPORT
		#endif
		#if (__GNUC__ == 4 && __GNUC_MINOR__ >= 8) || __GNUC__ > 4
			#define PLF_ALIGNMENT_SUPPORT
		#endif
		#if __GNUC__ >= 5 // GCC v4.9 and below do not support std::is_trivially_copyable
			#define PLF_TYPE_TRAITS_SUPPORT
		#endif
	#elif defined(__GLIBCXX__) // Using another compiler type with libstdc++ - we are assuming full c++11 compliance for compiler - which may not be true
		#if __GLIBCXX__ >= 20080606 	// libstdc++ 4.2 and below do not support variadic templates
			#define PLF_VARIADICS_SUPPORT
		#endif
		#if __GLIBCXX__ >= 20090421 	// libstdc++ 4.3 and below do not support initializer lists
			#define PLF_INITIALIZER_LIST_SUPPORT
		#endif
		#if __GLIBCXX__ >= 20160111
			#define PLF_ALLOCATOR_TRAITS_SUPPORT
			#define PLF_NOEXCEPT noexcept
			#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator) noexcept(std::allocator_traits<the_allocator>::propagate_on_container_move_assignment::value || std::allocator_traits<the_allocator>::is_always_equal::value)
			#define PLF_NOEXCEPT_SWAP(the_allocator) noexcept(std::allocator_traits<the_allocator>::propagate_on_container_swap::value || std::allocator_traits<the_allocator>::is_always_equal::value)
		#elif __GLIBCXX__ >= 20120322
			#define PLF_ALLOCATOR_TRAITS_SUPPORT
			#define PLF_NOEXCEPT noexcept
			#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator) noexcept
			#define PLF_NOEXCEPT_SWAP(the_allocator) noexcept
		#else
			#define PLF_NOEXCEPT throw()
			#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator)
			#define PLF_NOEXCEPT_SWAP(the_allocator)
		#endif
		#if __GLIBCXX__ >= 20130322
			#define PLF_ALIGNMENT_SUPPORT
		#endif
		#if __GLIBCXX__ >= 20150422 // libstdc++ v4.9 and below do not support std::is_trivially_copyable
			#define PLF_TYPE_TRAITS_SUPPORT
		#endif
	#elif defined(_LIBCPP_VERSION)
		#define PLF_ALLOCATOR_TRAITS_SUPPORT
		#define PLF_VARIADICS_SUPPORT
		#define PLF_INITIALIZER_LIST_SUPPORT
		#define PLF_ALIGNMENT_SUPPORT
		#define PLF_NOEXCEPT noexcept
		#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator) noexcept(std::allocator_traits<the_allocator>::propagate_on_container_move_assignment::value || std::allocator_traits<the_allocator>::is_always_equal::value)
		#define PLF_NOEXCEPT_SWAP(the_allocator) noexcept(std::allocator_traits<the_allocator>::propagate_on_container_swap::value || std::allocator_traits<the_allocator>::is_always_equal::value)

		#if !(defined(_LIBCPP_CXX03_LANG) || defined(_LIBCPP_HAS_NO_RVALUE_REFERENCES))
			#define PLF_TYPE_TRAITS_SUPPORT
		#endif
	#else // Assume type traits and initializer support for other compilers and standard libraries
		#define PLF_ALLOCATOR_TRAITS_SUPPORT
		#define PLF_ALIGNMENT_SUPPORT
		#define PLF_VARIADICS_SUPPORT
		#define PLF_INITIALIZER_LIST_SUPPORT
		#define PLF_TYPE_TRAITS_SUPPORT
		#define PLF_NOEXCEPT noexcept
		#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator) noexcept(std::allocator_traits<the_allocator>::propagate_on_container_move_assignment::value || std::allocator_traits<the_allocator>::is_always_equal::value)
		#define PLF_NOEXCEPT_SWAP(the_allocator) noexcept(std::allocator_traits<the_allocator>::propagate_on_container_swap::value || std::allocator_traits<the_allocator>::is_always_equal::value)
	#endif

	#if __cplusplus >= 201703L
		#if defined(__clang__) && ((__clang_major__ == 3 && __clang_minor__ == 9) || __clang_major__ > 3)
			#define PLF_CONSTEXPR constexpr
			#define PLF_CONSTEXPR_SUPPORT
		#elif defined(__GNUC__) && __GNUC__ >= 7
			#define PLF_CONSTEXPR constexpr
			#define PLF_CONSTEXPR_SUPPORT
		#elif !defined(__clang__) && !defined(__GNUC__)
			#define PLF_CONSTEXPR constexpr // assume correct C++17 implementation for other compilers
			#define PLF_CONSTEXPR_SUPPORT
		#else
			#define PLF_CONSTEXPR
		#endif
	#else
		#define PLF_CONSTEXPR
	#endif

	#if __cplusplus > 201703L // C++20
		#if defined(__clang__) && (__clang_major__ >= 10)
			#define PLF_CPP20_SUPPORT
		#elif defined(__GNUC__) && __GNUC__ >= 10
			#define PLF_CPP20_SUPPORT
		#elif !defined(__clang__) && !defined(__GNUC__) // assume correct C++20 implementation for other compilers
			#define PLF_CPP20_SUPPORT
		#endif
	#endif

	#define PLF_MOVE_SEMANTICS_SUPPORT
#else
	#define PLF_FORCE_INLINE
	#define PLF_NOEXCEPT throw()
	#define PLF_NOEXCEPT_SWAP(the_allocator)
	#define PLF_NOEXCEPT_MOVE_ASSIGNMENT(the_allocator)
	#define PLF_CONSTEXPR
#endif



#include <functional> // std::greater
#include <vector> // range-insert testing
#include <algorithm> // std::find
#include <cstdio> // log redirection, printf
#include <cstdlib> // abort

#ifdef PLF_MOVE_SEMANTICS_SUPPORT
	#include <utility> // std::move
#endif

#include "plf_colony.h"



void title1(const char *title_text)
{
	printf("\n\n\n*** %s ***\n", title_text);
	printf("===========================================\n\n\n");
}

void title2(const char *title_text)
{
	printf("\n\n--- %s ---\n\n", title_text);
}


void failpass(const char *test_type, bool condition)
{
	printf("%s: ", test_type);

	if (condition)
	{
		printf("Pass\n");
	}
	else
	{
		printf("Fail\n");
		getchar();
		abort();
	}
}


#ifdef PLF_VARIADICS_SUPPORT
	struct perfect_forwarding_test
	{
		const bool success;

		perfect_forwarding_test(int&& /*perfect1*/, int& perfect2)
			: success(true)
		{
			perfect2 = 1;
		}

		template <typename T, typename U>
		perfect_forwarding_test(T&& /*imperfect1*/, U&& /*imperfect2*/)
			: success(false)
		{}
	};



	struct small_struct
	{
		double *empty_field_1;
		double unused_number;
		unsigned int empty_field2;
		double *empty_field_3;
		int number;
		unsigned int empty_field4;

		small_struct(const int num) PLF_NOEXCEPT: number(num) {};
	};



	class non_copyable_type
	{
	private:
		int i;
		non_copyable_type(const non_copyable_type &); // non construction-copyable
		non_copyable_type& operator=(const non_copyable_type &); // non copyable
	public:
		non_copyable_type(int a) : i(a) {}
	};

#endif




int global_counter = 0;

struct small_struct_non_trivial
{
	double *empty_field_1;
	double unused_number;
	unsigned int empty_field2;
	double *empty_field_3;
	int number;
	unsigned int empty_field4;

	small_struct_non_trivial(const int num) PLF_NOEXCEPT: number(num) {};
	~small_struct_non_trivial() { ++global_counter; };
};





// Fast xorshift+128 random number generator function (original: https://codingforspeed.com/using-faster-psudo-random-generator-xorshift/)
unsigned int xor_rand()
{
	static unsigned int x = 123456789;
	static unsigned int y = 362436069;
	static unsigned int z = 521288629;
	static unsigned int w = 88675123;

	const unsigned int t = x ^ (x << 11);

	// Rotate the static values (w rotation in return statement):
	x = y;
	y = z;
	z = w;

	return w = w ^ (w >> 19) ^ (t ^ (t >> 8));
}



int main()
{
	freopen("error.log","w", stderr); // For catching assertion failure info when run outside of a command line prompt

	using namespace std;
	using namespace plf;


	for (unsigned int looper = 0; looper != 100; ++looper)
	{
		{
			title1("Colony");
			title2("Test Basics");

			colony<int *> p_colony;

			failpass("Colony empty", p_colony.empty());

			int ten = 10;
			p_colony.insert(&ten);

			failpass("Colony not-empty", !p_colony.empty());

			title2("Iterator tests");

			failpass("Begin() working", **p_colony.begin() == 10);
			failpass("End() working", p_colony.begin() != p_colony.end());


			p_colony.clear();

			failpass("Begin = End after clear", p_colony.begin() == p_colony.end());

			int twenty = 20;

			for (unsigned int temp = 0; temp != 200; ++temp)
			{
				p_colony.insert(&ten);
				p_colony.insert(&twenty);
			}

			int total = 0, numtotal = 0;

			for(colony<int *>::iterator the_iterator = p_colony.begin(); the_iterator != p_colony.end(); ++the_iterator)
			{
				++total;
				numtotal += **the_iterator;
			}

			failpass("Iteration count test", total == 400);
			failpass("Iterator access test", numtotal == 6000);

			colony<int *>::iterator plus_twenty = p_colony.begin();
			p_colony.advance(plus_twenty, 20);
			colony<int *>::iterator plus_two_hundred = p_colony.begin();
			p_colony.advance(plus_two_hundred, 200);

			failpass("Iterator + distance test", p_colony.distance(p_colony.begin(), plus_twenty) == 20);
			failpass("Iterator - distance test", p_colony.distance(plus_two_hundred, p_colony.begin()) == -200);

			colony<int *>::iterator next_iterator = p_colony.next(p_colony.begin(), 5);
			colony<int *>::const_iterator prev_iterator = p_colony.prev(p_colony.cend(), 300);

			failpass("Iterator next test", p_colony.distance(p_colony.begin(), next_iterator) == 5);
			failpass("Const iterator prev test", p_colony.distance(p_colony.cend(), prev_iterator) == -300);
			#if defined(__cplusplus) && __cplusplus >= 201402L
				colony<int *>::iterator prev_iterator2 = p_colony.prev(p_colony.end(), 300);
				failpass("Iterator/Const iterator equality operator test", prev_iterator == prev_iterator2);
			#endif

			prev_iterator = p_colony.begin();
			p_colony.advance(prev_iterator, 5);
			failpass("Iterator/Const iterator equality operator test 2", prev_iterator == next_iterator);

			colony<int *> p_colony2;
			p_colony2 = p_colony;
			colony<int *> p_colony3(p_colony);
			colony<int *> p_colony4(p_colony2, p_colony2.get_allocator());

			colony<int *>::iterator it1 = p_colony.begin();
			colony<int *>::const_iterator cit(it1);

			failpass("Copy test", p_colony2.size() == 400);
			failpass("Copy construct test", p_colony3.size() == 400);
			failpass("Allocator-extended copy construct test", p_colony4.size() == 400);


			failpass("Equality operator test", p_colony == p_colony2);
			failpass("Equality operator test 2", p_colony2 == p_colony3);

			p_colony2.insert(&ten);

			failpass("Inequality operator test", p_colony2 != p_colony3);

			numtotal = 0;
			total = 0;

			for (colony<int *>::reverse_iterator the_iterator = p_colony.rbegin(); the_iterator != p_colony.rend(); ++the_iterator)
			{
				++total;
				numtotal += **the_iterator;
			}


			failpass("Reverse iteration count test", total == 400);
			failpass("Reverse iterator access test", numtotal == 6000);

			colony<int *>::reverse_iterator r_iterator = p_colony.rbegin();
			p_colony.advance(r_iterator, 50);

			failpass("Reverse iterator advance and distance test", p_colony.distance(p_colony.rbegin(), r_iterator) == 50);

			colony<int *>::reverse_iterator r_iterator2 = p_colony.next(r_iterator, 2);

			failpass("Reverse iterator next and distance test", p_colony.distance(p_colony.rbegin(), r_iterator2) == 52);

			numtotal = 0;
			total = 0;

			for(colony<int *>::iterator the_iterator = p_colony.begin(); the_iterator < p_colony.end(); p_colony.advance(the_iterator, 2))
			{
				++total;
				numtotal += **the_iterator;
			}

			failpass("Multiple iteration test", total == 200);
			failpass("Multiple iteration access test", numtotal == 2000);

			numtotal = 0;
			total = 0;

			for(colony<int *>::const_iterator the_iterator = p_colony.cbegin(); the_iterator != p_colony.cend(); ++the_iterator)
			{
				++total;
				numtotal += **the_iterator;
			}

			failpass("Const_iterator test", total == 400);
			failpass("Const_iterator access test", numtotal == 6000);


			numtotal = 0;
			total = 0;

			for(colony<int *>::const_reverse_iterator the_iterator = --colony<int *>::const_reverse_iterator(p_colony.crend()); the_iterator != colony<int *>::const_reverse_iterator(p_colony.crbegin()); --the_iterator)
			{
				++total;
				numtotal += **the_iterator;
			}

			failpass("Const_reverse_iterator -- test", total == 399);
			failpass("Const_reverse_iterator -- access test", numtotal == 5980);

			total = 0;

			for(colony<int *>::iterator the_iterator = ++colony<int *>::iterator(p_colony.begin()); the_iterator < p_colony.end(); ++the_iterator)
			{
				++total;
				the_iterator = p_colony.erase(the_iterator);
			}

			failpass("Partial erase iteration test", total == 200);
			failpass("Post-erase size test", p_colony.size() == 200);

			const unsigned int temp_capacity = static_cast<unsigned int>(p_colony.capacity());
			p_colony.shrink_to_fit();
			failpass("Shrink_to_fit test", p_colony.capacity() < temp_capacity);
			failpass("Shrink_to_fit test 2", p_colony.capacity() == 200);

			total = 0;

			for(colony<int *>::reverse_iterator the_iterator = p_colony.rbegin(); the_iterator != p_colony.rend(); ++the_iterator)
			{
				colony<int *>::iterator it = the_iterator.base();
				the_iterator = p_colony.erase(--it);
				++total;
			}

			failpass("Full erase reverse iteration test", total == 200);
			failpass("Post-erase size test", p_colony.size() == 0);

			for (unsigned int temp = 0; temp != 200; ++temp)
			{
				p_colony.insert(&ten);
				p_colony.insert(&twenty);
			}

			total = 0;

			for(colony<int *>::iterator the_iterator = --colony<int *>::iterator(p_colony.end()); the_iterator != p_colony.begin(); --the_iterator)
			{
				++total;
			}

			failpass("Negative iteration test", total == 399);


			total = 0;

			for(colony<int *>::iterator the_iterator = --(colony<int *>::iterator(p_colony.end())); the_iterator != p_colony.begin(); p_colony.advance(the_iterator, -2))
			{
				++total;
			}

			failpass("Negative multiple iteration test", total == 200);

			#ifdef PLF_MOVE_SEMANTICS_SUPPORT
				p_colony2 = std::move(p_colony);
				failpass("Move test", p_colony2.size() == 400);

				p_colony.insert(&ten);

				failpass("Insert to post-moved-colony test", p_colony.size() == 1);

				colony<int *> p_colony5(p_colony2);
				colony<int *> p_colony6(std::move(p_colony5), p_colony2.get_allocator());

				failpass("Allocator-extended move construct test", p_colony6.size() == 400);
			#else
				p_colony2 = p_colony;
			#endif

			p_colony3 = p_colony2;

			failpass("Copy test 2", p_colony3.size() == 400);

			p_colony2.insert(&ten);

			p_colony2.swap(p_colony3);

			failpass("Swap test", p_colony2.size() == p_colony3.size() - 1);

			swap(p_colony2, p_colony3);

			failpass("Swap test 2", p_colony3.size() == p_colony2.size() - 1);

			failpass("max_size() test", p_colony2.max_size() > p_colony2.size());

		}


		{
			title2("Iterator comparison tests");

			colony<int> i_colony;

			for (int temp = 0; temp != 10; ++temp)
			{
				i_colony.insert(temp);
			}

			colony<int>::iterator it1 = i_colony.begin(), it2 = i_colony.begin();

			++it2;
			++it2;
			++it2;

			failpass("Iterator ++ test", *it2 == 3);

			failpass("Iterator > test", it2 > it1);

			failpass("Iterator >= test", it2 >= it1);

			failpass("Iterator < test", it1 < it2);

			failpass("Iterator <= test", it1 <= it2);

			failpass("Iterator != test", it2 != it1);

			#ifdef PLF_CPP20_SUPPORT
				failpass("Iterator <=> test 1", (it2 <=> it1) == 1);

				failpass("Iterator <=> test 2", (it1 <=> it2) == -1);

				it1 = it2;

				failpass("Iterator <=> test 3", (it1 <=> it2) == 0);
			#endif
		}


		{
			title2("Insert and Erase tests");

			colony<int> i_colony;

			for (int temp = 0; temp != 500000; ++temp)
			{
				i_colony.insert(temp);
			}


			failpass("Size after insert test", i_colony.size() == 500000);


			colony<int>::iterator found_item = std::find(i_colony.begin(), i_colony.end(), 5000);;

			failpass("std::find iterator test", *found_item == 5000);


			colony<int>::reverse_iterator found_item2 = std::find(i_colony.rbegin(), i_colony.rend(), 5000);;

			failpass("std::find reverse_iterator test", *found_item2 == 5000);


			for (colony<int>::iterator the_iterator = i_colony.begin(); the_iterator != i_colony.end(); ++the_iterator)
			{
				the_iterator = i_colony.erase(the_iterator);
			}

			failpass("Erase alternating test", i_colony.size() == 250000);

			do
			{
				for (colony<int>::iterator the_iterator = i_colony.begin(); the_iterator != i_colony.end();)
				{
					if ((xor_rand() & 7) == 0)
					{
						the_iterator = i_colony.erase(the_iterator);
					}
					else
					{
						++the_iterator;
					}
				}

			} while (!i_colony.empty());

			failpass("Erase randomly till-empty test", i_colony.size() == 0);


			i_colony.clear();
			i_colony.set_minimum_block_capacity(10000);

			i_colony.insert(30000, 1); // fill-insert 30000 elements

			failpass("Size after reinitialize + fill-insert test", i_colony.size() == 30000);

			unsigned short count2 = 0;

			do
			{
				for (colony<int>::iterator the_iterator = i_colony.begin(); the_iterator != i_colony.end();)
				{
					if ((xor_rand() & 7) == 0)
					{
						the_iterator = i_colony.erase(the_iterator);
						++count2;
					}
					else
					{
						++the_iterator;
					}
				}

			} while (count2 < 15000);

			failpass("Erase randomly till half-empty test", i_colony.size() == 30000u - count2);

			i_colony.insert(count2, 1);

			failpass("Size after reinsert test", i_colony.size() == 30000);




			unsigned int sum = 0;

			for (colony<int>::iterator the_iterator = i_colony.begin(); the_iterator != i_colony.end();)
			{
				if (++sum == 3)
				{
					sum = 0;
					the_iterator = i_colony.erase(the_iterator);
				}
				else
				{
					i_colony.insert(1);
					++the_iterator;
				}
			}

			failpass("Alternating insert/erase test", i_colony.size() == 45001);


			do
			{
				for (colony<int>::iterator the_iterator = i_colony.begin(); the_iterator != i_colony.end();)
				{
					if ((xor_rand() & 3) == 0)
					{
						++the_iterator;
						i_colony.insert(1);
					}
					else
					{
						the_iterator = i_colony.erase(the_iterator);
					}
				}
			} while (!i_colony.empty());;

			failpass("Random insert/erase till empty test", i_colony.size() == 0);


			i_colony.insert(500000, 10);

			failpass("Insert post-erase test", i_colony.size() == 500000);
			colony<int>::iterator it2 = i_colony.begin();
			i_colony.advance(it2, 250000);


			for (; it2 != i_colony.end();)
			{
				it2 = i_colony.erase(it2);
			}

			failpass("Large multi-increment iterator test", i_colony.size() == 250000);

			i_colony.insert(250000, 10);

			colony<int>::iterator end_iterator = i_colony.end();
			i_colony.advance(end_iterator, -250000);

			for (colony<int>::iterator the_iterator = i_colony.begin(); the_iterator != end_iterator;)
			{
				the_iterator = i_colony.erase(the_iterator);
			}

			failpass("Large multi-decrement iterator test", i_colony.size() == 250000);


			i_colony.insert(250000, 10);
			int total = 0;

			for (colony<int>::iterator the_iterator = i_colony.begin(); the_iterator != i_colony.end(); ++the_iterator)
			{
				total += *the_iterator;
			}

			failpass("Re-insert post-heavy-erasure test", total == 5000000);


			end_iterator = i_colony.end();
			i_colony.advance(end_iterator, -50001);
			colony<int>::iterator begin_iterator = i_colony.begin();
			i_colony.advance(begin_iterator, 300000);

			for (colony<int>::iterator the_iterator = begin_iterator; the_iterator != end_iterator;)
			{
				the_iterator = i_colony.erase(the_iterator);
			}

			failpass("Non-end decrement + erase test", i_colony.size() == 350001);


			i_colony.insert(100000, 10);

			begin_iterator = i_colony.begin();
			i_colony.advance(begin_iterator, 300001);


			for (colony<int>::iterator the_iterator = begin_iterator; the_iterator != i_colony.end();)
			{
				the_iterator = i_colony.erase(the_iterator);
			}

			failpass("Non-beginning increment + erase test", i_colony.size() == 300001);

			colony<int>::iterator temp_iterator = i_colony.begin();
			i_colony.advance(temp_iterator, 20); // Advance test 1

			unsigned int index = static_cast<unsigned int>(i_colony.get_index_from_iterator(temp_iterator));
			failpass("Advance + iterator-to-index test", index == 20);

			i_colony.erase(temp_iterator);
			temp_iterator = i_colony.begin(); // Check edge-case with advance when erasures present in initial group
			i_colony.advance(temp_iterator, 500);

			index = static_cast<unsigned int>(i_colony.get_index_from_iterator(temp_iterator));

			failpass("Advance + iterator-to-index test", index == 500);

			colony<int>::iterator temp2 = i_colony.get_iterator_from_pointer(&(*temp_iterator));

			failpass("Pointer-to-iterator test", temp2 != i_colony.end());

			temp2 = i_colony.get_iterator_from_index(500);

			failpass("Index-to-iterator test", temp2 == temp_iterator);


			for (colony<int>::iterator the_iterator = i_colony.begin(); the_iterator != i_colony.end();)
			{
				the_iterator = i_colony.erase(the_iterator);
			}

			failpass("Total erase test", i_colony.empty());


			i_colony.clear();
			i_colony.set_minimum_block_capacity(3);

			const unsigned int temp_capacity2 = static_cast<unsigned int>(i_colony.capacity());
			i_colony.reserve(1000);
			failpass("Colony reserve test", temp_capacity2 != i_colony.capacity());
			failpass("Colony reserve test2", i_colony.capacity() == 1000);

			unsigned int count = 0;

			for (unsigned int loop1 = 0; loop1 != 50000; ++loop1)
			{
				for (unsigned int loop = 0; loop != 10; ++loop)
				{
					if ((xor_rand() & 7) == 0)
					{
						i_colony.insert(1);
						++count;
					}
				}

				unsigned int internal_loop_counter = 0;

				for (colony<int>::iterator the_iterator = i_colony.begin(); the_iterator != i_colony.end();)
				{
					if ((xor_rand() & 7) == 0)
					{
						the_iterator = i_colony.erase(the_iterator);
						--count;
					}
					else
					{
						++the_iterator;
					}

					++internal_loop_counter;
				}
			}

			failpass("Multiple sequential small insert/erase commands test", count == i_colony.size());
		}


		{
			title2("Range-erase tests");

			colony<int> i_colony;

			int counter = 0;

			for (; counter != 1000; ++counter)
			{
				i_colony.insert(counter);
			}


			colony<int>::iterator it1 = i_colony.begin(), it2 = i_colony.begin();

			i_colony.advance(it1, 500);
			i_colony.advance(it2, 800);

			i_colony.erase(it1, it2);

			counter = 0;

			for (colony<int>::iterator it = i_colony.begin(); it != i_colony.end(); ++it)
			{
				++counter;
			}

			failpass("Simple range-erase test 1", counter == 700 && i_colony.size() == 700);


			it1 = it2 = i_colony.begin();

			i_colony.advance(it1, 400);
			i_colony.advance(it2, 500); // This should put it2 past the point of previous erasures

			i_colony.erase(it1, it2);

			counter = 0;

			for (colony<int>::iterator it = i_colony.begin(); it != i_colony.end(); ++it)
			{
				++counter;
			}

			failpass("Simple range-erase test 2", counter == 600 && i_colony.size() == 600);



			it2 = it1 = i_colony.begin();

			i_colony.advance(it1, 4);
			i_colony.advance(it2, 9); // This should put it2 past the point of previous erasures

			i_colony.erase(it1, it2);

			counter = 0;

			for (colony<int>::iterator it = i_colony.begin(); it != i_colony.end(); ++it)
			{
				++counter;
			}

			failpass("Simple range-erase test 3", counter == 595 && i_colony.size() == 595);




			it2 = it1 = i_colony.begin();

			i_colony.advance(it2, 50);

			i_colony.erase(it1, it2);

			counter = 0;

			for (colony<int>::iterator it = i_colony.begin(); it != i_colony.end(); ++it)
			{
				++counter;
			}

			failpass("Range-erase from begin() test 1", counter == 545 && i_colony.size() == 545);




			it1 = i_colony.begin();
			it2 = i_colony.end();

			i_colony.advance(it1, 345); // Test erasing and validity when it removes the final group in colony
			i_colony.erase(it1, it2);

			counter = 0;

			for (colony<int>::iterator it = i_colony.begin(); it != i_colony.end(); ++it)
			{
				++counter;
			}

			failpass("Range-erase to end() test 1", counter == 345 && i_colony.size() == 345);



			i_colony.clear();

			for (counter = 0; counter != 3000; ++counter)
			{
				i_colony.insert(counter);
			}

			for (colony<int>::iterator it = i_colony.begin(); it < i_colony.end(); ++it)
			{
				it = i_colony.erase(it);
			}

			it2 = it1 = i_colony.begin();

			i_colony.advance(it1, 4);
			i_colony.advance(it2, 600);
			i_colony.erase(it1, it2);

			counter = 0;

			for (colony<int>::iterator it = i_colony.begin(); it != i_colony.end(); ++it)
			{
				++counter;
			}

			failpass("Range-erase with colony already half-erased, alternating erasures", counter == 904 && i_colony.size() == 904);



			i_colony.clear();

			for (counter = 0; counter != 3000; ++counter)
			{
				i_colony.insert(counter);
			}

			for (colony<int>::iterator it = i_colony.begin(); it < i_colony.end(); ++it)
			{
				if ((xor_rand() & 1) == 0)
				{
					it = i_colony.erase(it);
				}
			}

			if (i_colony.size() < 400)
			{
				for (counter = 0; counter != 400; ++counter)
				{
					i_colony.insert(counter);
				}
			}

			it1 = i_colony.begin();
			it2 = i_colony.end();

			i_colony.advance(it1, 400);
			i_colony.erase(it1, it2);

			counter = 0;

			for (colony<int>::iterator it = i_colony.begin(); it != i_colony.end(); ++it)
			{
				++counter;
			}

			failpass("Range-erase with colony already third-erased, randomized erasures", counter == 400 && i_colony.size() == 400);



			unsigned int size, range1, range2, internal_loop_counter;

			for (unsigned int loop_counter = 0; loop_counter != 50; ++loop_counter)
			{
				i_colony.clear();

				for (counter = 0; counter != 1000; ++counter)
				{
					i_colony.insert(counter);
				}

				internal_loop_counter = 0;

				while (!i_colony.empty())
				{
					it2 = it1 = i_colony.begin();

					size = static_cast<unsigned int>(i_colony.size());
					range1 = xor_rand() % size;
					range2 = range1 + 1 + (xor_rand() % (size - range1));
					i_colony.advance(it1, static_cast<int>(range1));
					i_colony.advance(it2, static_cast<int>(range2));

					i_colony.erase(it1, it2);

					counter = 0;

					for (colony<int>::iterator it = i_colony.begin(); it != i_colony.end(); ++it)
					{
						++counter;
					}

					if (i_colony.size() != static_cast<unsigned int>(counter))
					{
						printf("Fuzz-test range-erase randomly Fail: loop counter: %u, internal_loop_counter: %u.\n", loop_counter, internal_loop_counter);
						getchar();
						abort();
					}

					if (i_colony.size() != i_colony.group_size_sum())
					{
						printf("Fuzz-test range-erase randomly Fail - group_size_sum failure: loop counter: %u, internal_loop_counter: %u, size: %u, group_size_sum: %u.\n", loop_counter, internal_loop_counter, static_cast<unsigned int>(i_colony.size()), static_cast<unsigned int>(i_colony.group_size_sum()));
						getchar();
						abort();
					}

					if (i_colony.size() > 2)
					{ // Test to make sure our stored erased_locations are valid
						i_colony.insert(1);
						i_colony.insert(10);
					}

					++internal_loop_counter;
				}
			}

			failpass("Fuzz-test range-erase randomly until empty", i_colony.size() == 0);



			for (unsigned int loop_counter = 0; loop_counter != 50; ++loop_counter)
			{
				i_colony.clear();
				internal_loop_counter = 0;

				i_colony.insert(10000, 10);

				while (!i_colony.empty())
				{
					it2 = it1 = i_colony.begin();

					size = static_cast<unsigned int>(i_colony.size());
					range1 = xor_rand() % size;
					range2 = range1 + 1 + (xor_rand() % (size - range1));
					i_colony.advance(it1, static_cast<int>(range1));
					i_colony.advance(it2, static_cast<int>(range2));

					i_colony.erase(it1, it2);

					counter = 0;

					for (colony<int>::iterator it = i_colony.begin(); it != i_colony.end(); ++it)
					{
						++counter;
					}

					if (i_colony.size() != i_colony.group_size_sum())
					{
						printf("Fuzz-test range-erase + fill-insert randomly Fails during erase - group_size_sum failure: loop counter: %u, internal_loop_counter: %u, size: %u, group_size_sum: %u.\n", loop_counter, internal_loop_counter, static_cast<unsigned int>(i_colony.size()), static_cast<unsigned int>(i_colony.group_size_sum()));
						getchar();
						abort();
					}

					if (i_colony.size() != static_cast<unsigned int>(counter))
					{
						printf("Fuzz-test range-erase + fill-insert randomly Fails during erase: loop counter: %u, internal_loop_counter: %u.\n", loop_counter, internal_loop_counter);
						getchar();
						abort();
					}

					if (i_colony.size() > 100)
					{ // Test to make sure our stored erased_locations are valid & fill-insert is functioning properly in these scenarios
						const unsigned int extra_size = xor_rand() % 128;
						i_colony.insert(extra_size, 5);

						if (i_colony.size() != i_colony.group_size_sum())
						{
							printf("Fuzz-test range-erase + fill-insert randomly Fails during insert - group_size_sum failure: loop counter: %u, internal_loop_counter: %u, size: %u, group_size_sum: %u.\n", loop_counter, internal_loop_counter, static_cast<unsigned int>(i_colony.size()), static_cast<unsigned int>(i_colony.group_size_sum()));
							getchar();
							abort();
						}

						if (i_colony.size() != static_cast<unsigned int>(counter) + extra_size)
						{
							printf("Fuzz-test range-erase + fill-insert randomly Fails during fill-insert: loop counter: %u, internal_loop_counter: %u.\n", loop_counter, internal_loop_counter);
							getchar();
							abort();
						}

						counter = 0;

						for (colony<int>::iterator it = i_colony.begin(); it != i_colony.end(); ++it)
						{
							++counter;
						}

						if (i_colony.size() != static_cast<unsigned int>(counter))
						{
							printf("Fuzz-test range-erase + fill-insert randomly Fails during counter-test fill-insert: loop counter: %u, internal_loop_counter: %u.\n", loop_counter, internal_loop_counter);
							getchar();
							abort();
						}
					}

					++internal_loop_counter;
				}
			}

			failpass("Fuzz-test range-erase + fill-insert randomly until empty", i_colony.size() == 0);

			i_colony.erase(i_colony.begin(), i_colony.end());

			failpass("Range-erase when colony is empty test (crash test)", i_colony.size() == 0);

			i_colony.insert(10, 1);

			i_colony.erase(i_colony.begin(), i_colony.begin());

			failpass("Range-erase when range is empty test (crash test)", i_colony.size() == 10);



			i_colony.insert(10000, 5);

			int sum1 = 0, sum2 = 0;
			range1 = 0;
			range2 = 0;


			// Erase half of all elements and sum the rest:
			for (colony<int>::iterator it = i_colony.begin(); it != i_colony.end(); ++it)
			{
				it = i_colony.erase(it);
				sum1 += *it;
				++range1;
			}


			colony<int>::raw_memory_block_pointers *data = i_colony.data();

			// Manually sum using raw memory blocks:
			for (unsigned int block_num = 0; block_num != data->number_of_blocks; ++block_num)
			{
				for (unsigned short block_sub_index = 0; block_sub_index != data->block_capacities[block_num]; ++block_sub_index)
				{
					if ((data->skipfield_memory_block_pointers[block_num])[block_sub_index] == 0)
					{
						// We have to reinterpret_cast (via pointers) back to the original type since colony stores data internally as an aligned version of the same type
						sum2 += *reinterpret_cast<int *>((data->element_memory_block_pointers[block_num]) + block_sub_index);
						++range2;
					}
				}
			}

			delete data;

			failpass("Manual summing pass over elements gotten from data()", (sum1 == sum2) && (range1 == range2));
		}



		{
			title1("Non-trivial type tests");

			colony<small_struct_non_trivial> ss_nt;
			colony<small_struct_non_trivial>::iterator ss_it1, ss_it2;

			small_struct_non_trivial ss(5);

			unsigned int size, range1 = 0, range2 = 0, internal_loop_counter;
			int counter, sum1 = 0, sum2 = 0;

			ss_nt.insert(10000, ss);

			failpass("Non-trivial type insert test", ss_nt.size() == 10000);


			for (colony<small_struct_non_trivial>::iterator ss_it = ss_nt.begin(); ss_it != ss_nt.end(); ++ss_it)
			{
				ss_it = ss_nt.erase(ss_it);
				sum1 += ss_it->number;
				++range1;
			}

			failpass("Non-trivial type erase half of all elements", ss_nt.size() == 5000);


			colony<small_struct_non_trivial>::raw_memory_block_pointers *data = ss_nt.data();

			// Manually pass over contents:
			for (unsigned int block_num = 0; block_num != data->number_of_blocks; ++block_num)
			{
				for (unsigned short block_sub_index = 0; block_sub_index != data->block_capacities[block_num]; ++block_sub_index)
				{
					if (*(data->skipfield_memory_block_pointers[block_num] + block_sub_index) == 0)
					{
						// We have to reinterpret_cast (via pointers) back to the original type since the colony stores data as an aligned version of the same type
						sum2 += (reinterpret_cast<small_struct_non_trivial *>(data->element_memory_block_pointers[block_num] + block_sub_index))->number;
						++range2;
					}
				}
			}

			delete data;

			failpass("Non-trivial manual summing pass over elements gotten from data()", (sum1 == sum2) && (range1 == range2));


			for (unsigned int loop_counter = 0; loop_counter != 50; ++loop_counter)
			{
				ss_nt.clear();

				for (counter = 0; counter != 1000; ++counter)
				{
					ss_nt.insert(counter);
				}

				internal_loop_counter = 0;

				while (!ss_nt.empty())
				{
					ss_it2 = ss_it1 = ss_nt.begin();

					size = static_cast<unsigned int>(ss_nt.size());
					range1 = xor_rand() % size;
					range2 = range1 + 1 + (xor_rand() % (size - range1));
					ss_nt.advance(ss_it1, static_cast<int>(range1));
					ss_nt.advance(ss_it2, static_cast<int>(range2));

					ss_nt.erase(ss_it1, ss_it2);

					counter = 0;

					for (colony<small_struct_non_trivial>::iterator ss_it = ss_nt.begin(); ss_it != ss_nt.end(); ++ss_it)
					{
						++counter;
					}

					if (ss_nt.size() != static_cast<unsigned int>(counter))
					{
						printf("Fuzz-test range-erase randomly Fail: loop counter: %u, internal_loop_counter: %u.\n", loop_counter, internal_loop_counter);
						getchar();
						abort();
					}

					if (ss_nt.size() != ss_nt.group_size_sum())
					{
						printf("Fuzz-test range-erase randomly Fail - group_size_sum failure: loop counter: %u, internal_loop_counter: %u, size: %u, group_size_sum: %u.\n", loop_counter, internal_loop_counter, static_cast<unsigned int>(ss_nt.size()), static_cast<unsigned int>(ss_nt.group_size_sum()));
						getchar();
						abort();
					}

					if (ss_nt.size() > 2)
					{ // Test to make sure our stored erased_locations are valid
						ss_nt.insert(1);
						ss_nt.insert(10);
					}

					++internal_loop_counter;
				}
			}

			failpass("Non-trivial type fuzz-test range-erase randomly until empty", ss_nt.size() == 0);
		}


		{
			title2("Sort tests");

			colony<int> i_colony;

			i_colony.reserve(50000);

			for (unsigned int temp = 0; temp != 50000; ++temp)
			{
				i_colony.insert(xor_rand() & 65535);
			}

			i_colony.sort();

			bool sorted = true;
			int previous = 0;

			for (colony<int>::iterator current = i_colony.begin(); current != i_colony.end(); ++current)
			{
				if (previous > *current)
				{
					sorted = false;
					break;
				}

				previous = *current;
			}

			failpass("Less-than sort test", sorted);

			i_colony.sort(std::greater<int>());

			previous = 65536;

			for (colony<int>::iterator current = i_colony.begin(); current != i_colony.end(); ++current)
			{
				if (previous < *current)
				{
					sorted = false;
					break;
				}

				previous = *current;
			}

			failpass("Greater-than sort test", sorted);
		}



		{
			title2("Different insertion-style tests");

			#ifdef PLF_INITIALIZER_LIST_SUPPORT
				colony<int> i_colony = {1, 2, 3};

				failpass("Initializer-list constructor test", i_colony.size() == 3);
			#else
				colony<int> i_colony(3, 1);
			#endif

			colony<int> i_colony2(i_colony.begin(), i_colony.end());

			failpass("Range constructor test", i_colony2.size() == 3);

			colony<int> i_colony3(5000, 2, 100, 1000);

			failpass("Fill construction test", i_colony3.size() == 5000);

			i_colony2.insert(500000, 5);

			failpass("Fill insertion test", i_colony2.size() == 500003);

			std::vector<int> some_ints(500, 2);

			i_colony2.insert(some_ints.begin(), some_ints.end());

			failpass("Range insertion test", i_colony2.size() == 500503);

			i_colony3.clear();
			i_colony2.clear();
			i_colony2.reserve(50000);
			i_colony2.insert(60000, 1);

			int total = 0;

			for (colony<int>::iterator it = i_colony2.begin(); it != i_colony2.end(); ++it)
			{
				total += *it;
			}

			failpass("Reserve + fill insert test", i_colony2.size() == 60000 && total == 60000);


			i_colony2.clear();
			i_colony2.reserve(5000);
			i_colony2.insert(60, 1);

			total = 0;

			for (colony<int>::iterator it = i_colony2.begin(); it != i_colony2.end(); ++it)
			{
				total += *it;
			}

			failpass("Reserve + fill insert test 2", i_colony2.size() == 60 && total == 60);

			i_colony2.insert(6000, 1);

			total = 0;

			for (colony<int>::iterator it = i_colony2.begin(); it != i_colony2.end(); ++it)
			{
				total += *it;
			}

			failpass("Reserve + fill + fill test", i_colony2.size() == 6060 && total == 6060);

			i_colony2.reserve(18000);
			i_colony2.insert(6000, 1);

			total = 0;

			for (colony<int>::iterator it = i_colony2.begin(); it != i_colony2.end(); ++it)
			{
				total += *it;
			}

			failpass("Reserve + fill + fill + reserve + fill test", i_colony2.size() == 12060 && total == 12060);


		}


		#ifdef PLF_VARIADICS_SUPPORT
		{
			title2("Perfect Forwarding tests");

			colony<perfect_forwarding_test> pf_colony;

			int lvalue = 0;
			int &lvalueref = lvalue;

			pf_colony.emplace(7, lvalueref);

			failpass("Perfect forwarding test", (*pf_colony.begin()).success);
			failpass("Perfect forwarding test 2", lvalueref == 1);
		}


		{
			title2("Basic emplace test");

			colony<small_struct> ss_colony;
			int total1 = 0, total2 = 0;

			for (int counter = 0; counter != 100; ++counter)
			{
				ss_colony.emplace(counter);
				total1 += counter;
			}

			for (colony<small_struct>::iterator it = ss_colony.begin(); it != ss_colony.end(); ++it)
			{
				total2 += it->number;
			}

			failpass("Basic emplace test", total1 == total2);
			failpass("Basic emplace test 2", ss_colony.size() == 100);
		}


		{
			title2("Non-copyable type test");

			plf::colony<non_copyable_type> temp;

			temp.emplace(1);
			temp.emplace(2);

			failpass("Non-copyable size test", temp.size() == 2);
		}


		#endif


		{
			title2("Misc function tests");

			colony<int> colony1;
			colony1.set_block_capacity_limits(50, 100);

			colony1.insert(27);

			failpass("Change_group_sizes min-size test", colony1.capacity() == 50);

			for (int counter = 0; counter != 100; ++counter)
			{
				colony1.insert(counter);
			}

			failpass("Change_group_sizes max-size test", colony1.capacity() == 200);

			colony1.reinitialize(200, 2000);

			colony1.insert(27);

			failpass("Reinitialize min-size test", colony1.capacity() == 200);

			for (int counter = 0; counter != 3300; ++counter)
			{
				colony1.insert(counter);
			}

			failpass("Reinitialize max-size test", colony1.capacity() == 5200);

			colony1.set_block_capacity_limits(500, 500);

			failpass("Change_group_sizes resize test", colony1.capacity() == 3500);

			colony1.set_minimum_block_capacity(200);
			colony1.set_maximum_block_capacity(200);

			failpass("Change_maximum_group_size resize test", colony1.capacity() == 3400);

		}

		{
			title2("Splice tests");

			{
				colony<int> colony1, colony2;

				for(int number = 0; number != 20; ++number)
				{
					colony1.insert(number);
					colony2.insert(number + 20);
				}

				colony1.splice(colony2);

				int check_number = 0;
				bool fail = false;

				for (colony<int>::iterator current = colony1.begin(); current != colony1.end(); ++current)
				{
					if (check_number++ != *current)
					{
						fail = true;
					}
				}

				failpass("Small splice test 1", fail == false);
			}


			{
				colony<int> colony1, colony2;

				for(int number = 0; number != 100; ++number)
				{
					colony1.insert(number);
					colony2.insert(number + 100);
				}

				colony1.splice(colony2);

				int check_number = 0;
				bool fail = false;

				for (colony<int>::iterator current = colony1.begin(); current != colony1.end(); ++current)
				{
					if (check_number++ != *current)
					{
						fail = true;
					}
				}

				failpass("Small splice test 2", fail == false);
			}


			{
				colony<int> colony1, colony2;

				for(int number = 0; number != 100000; ++number)
				{
					colony1.insert(number);
					colony2.insert(number + 100000);
				}

				colony1.splice(colony2);

				int check_number = 0;
				bool fail = false;

				for (colony<int>::iterator current = colony1.begin(); current != colony1.end(); ++current)
				{
					if (check_number++ != *current)
					{
						fail = true;
					}
				}

				failpass("Large splice test 1", fail == false);
			}


			{
				colony<int> colony1, colony2;

				for(int number = 0; number != 100; ++number)
				{
					colony1.insert(number);
					colony2.insert(number + 100);
				}


				for (colony<int>::iterator current = colony2.begin(); current != colony2.end();)
				{
					if ((xor_rand() & 7) == 0)
					{
						current = colony2.erase(current);
					}
					else
					{
						++current;
					}
				}


				colony1.splice(colony2);

				int check_number = -1;
				bool fail = false;

				for (colony<int>::iterator current = colony1.begin(); current != colony1.end(); ++current)
				{
					if (check_number >= *current)
					{
						fail = true;
					}

					check_number = *current;
				}

				failpass("Erase + splice test 1", fail == false);
			}



			{
				colony<int> colony1, colony2;

				for(int number = 0; number != 100; ++number)
				{
					colony1.insert(number);
					colony2.insert(number + 100);
				}



				for (colony<int>::iterator current = colony2.begin(); current != colony2.end();)
				{
					if ((xor_rand() & 3) == 0)
					{
						current = colony2.erase(current);
					}
					else
					{
						++current;
					}
				}


				for (colony<int>::iterator current = colony1.begin(); current != colony1.end();)
				{
					if ((xor_rand() & 1) == 0)
					{
						current = colony1.erase(current);
					}
					else
					{
						++current;
					}
				}


				colony1.splice(colony2);

				int check_number = -1;
				bool fail = false;

				for (colony<int>::iterator current = colony1.begin(); current != colony1.end(); ++current)
				{
					if (check_number >= *current)
					{
						fail = true;
					}

					check_number = *current;
				}

				failpass("Erase + splice test 2", fail == false);
			}



			{
				colony<int> colony1, colony2;

				colony1.set_block_capacity_limits(200, 200);
				colony2.set_block_capacity_limits(200, 200);

				for(int number = 0; number != 100; ++number)
				{
					colony1.insert(number + 150);
				}


				for(int number = 0; number != 150; ++number)
				{
					colony2.insert(number);
				}


				colony1.splice(colony2);

				int check_number = -1;
				bool fail = false;

				for (colony<int>::iterator current = colony1.begin(); current != colony1.end(); ++current)
				{
					if (check_number >= *current)
					{
						fail = true;
					}

					check_number = *current;
				}

				failpass("Unequal size splice test 1", fail == false);
			}



			{
				colony<int> colony1, colony2;

				colony1.reinitialize(200, 200);
				colony2.reinitialize(200, 200);

				for(int number = 0; number != 100; ++number)
				{
					colony1.insert(100 - number);
				}


				for(int number = 0; number != 150; ++number)
				{
					colony2.insert(250 - number);
				}


				colony1.splice(colony2);

				int check_number = 255;
				bool fail = false;

				for (colony<int>::iterator current = colony1.begin(); current != colony1.end(); ++current)
				{
					if (check_number < *current)
					{
						fail = true;
					}

					check_number = *current;
				}

				failpass("Unequal size splice test 2", fail == false);
			}



			{
				colony<int> colony1, colony2;

				for(int number = 0; number != 100000; ++number)
				{
					colony1.insert(number + 200000);
				}


				for(int number = 0; number != 200000; ++number)
				{
					colony2.insert(number);
				}

				for (colony<int>::iterator current = colony2.begin(); current != colony2.end();)
				{
					if ((xor_rand() & 1) == 0)
					{
						current = colony2.erase(current);
					}
					else
					{
						++current;
					}
				}


				for (colony<int>::iterator current = colony1.begin(); current != colony1.end();)
				{
					if ((xor_rand() & 1) == 0)
					{
						current = colony1.erase(current);
					}
					else
					{
						++current;
					}
				}


				colony1.erase(--(colony1.end()));
				colony2.erase(--(colony2.end()));

				colony1.splice(colony2); // splice should swap the order at this point due to differences in numbers of unused elements at end of final group in each colony

				int check_number = -1;
				bool fail = false;

				for (colony<int>::iterator current = colony1.begin(); current != colony1.end(); ++current)
				{
					if (check_number >= *current)
					{
						fail = true;
						break;
					}

					check_number = *current;
				}

				failpass("Large unequal size + erase splice test 1", fail == false);


				do
				{
					for (colony<int>::iterator current = colony1.begin(); current != colony1.end();)
					{
						if ((xor_rand() & 3) == 0)
						{
							current = colony1.erase(current);
						}
						else if ((xor_rand() & 7) == 0)
						{
							colony1.insert(433);
							++current;
						}
						else
						{
							++current;
						}
					}

				} while (!colony1.empty());

				failpass("Post-splice insert-and-erase randomly till-empty test", colony1.size() == 0);
			}
		}


	}
	title1("Test Suite PASS - Press ENTER to Exit");
	getchar();

	return 0;
}
