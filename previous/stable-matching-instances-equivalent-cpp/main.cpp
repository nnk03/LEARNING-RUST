#include <algorithm>
#include <cmath>
#include <cstdint>
#include <deque>
#include <functional>
#include <iostream>
#include <optional>
#include <vector>

// Forward declaration of prefs_for
template <typename T, typename N, typename V>
std::deque<size_t> prefs_for(const T& person,
                             const std::vector<T>& others,
                             const V& value_func);

// Function to calculate preferences for all proposers
template <typename T, typename N, typename V>
std::vector<std::deque<size_t>> prefs_for_all(const std::vector<T>& people,
                                              const std::vector<T>& others,
                                              const V& value_func)
{
   std::vector<std::deque<size_t>> result;
   for (const auto& person : people)
   {
      // Explicitly specify N when calling prefs_for
      result.push_back(prefs_for<T, N>(person, others, value_func));
   }
   return result;
}

// Function to calculate preferences for a single person
template <typename T, typename N, typename V>
std::deque<size_t> prefs_for(const T& person,
                             const std::vector<T>& others,
                             const V& value_func)
{
   std::vector<std::pair<N, size_t>> valuations;
   for (size_t i = 0; i < others.size(); ++i)
   {
      valuations.push_back({value_func(person, others[i]), i});
   }
   std::sort(valuations.begin(), valuations.end());

   std::deque<size_t> prefs;
   for (const auto& valuation : valuations)
   {
      prefs.push_back(valuation.second);
   }
   return prefs;
}

// Function to perform the stable matching algorithm (asymmetric)
template <typename T, typename N, typename PV, typename RV>
std::vector<std::pair<size_t, size_t>> stable_matching_asymmetric(
    const std::vector<T>& proposers,
    const PV& proposer_value_func,
    const std::vector<T>& receivers,
    const RV& receiver_value_func)
{

   // Explicitly define N as the return type of the value function (lambda)
   auto proposer_prefs = prefs_for_all<T, N>(proposers, receivers, proposer_value_func);

   // Initialize receiver's fiancee list (holds the proposer and the associated value)
   std::vector<std::optional<std::pair<size_t, N>>> receiver2fiance(receivers.size());

   // Queue of single proposers (those who still need to propose)
   std::deque<size_t> single_proposers;
   for (size_t i = 0; i < proposers.size(); ++i)
   {
      single_proposers.push_back(i);
   }

   while (!single_proposers.empty())
   {
      size_t proposer = single_proposers.front();
      single_proposers.pop_front();

      if (!proposer_prefs[proposer].empty())
      {
         size_t receiver = proposer_prefs[proposer].front();
         proposer_prefs[proposer].pop_front();

         if (receiver2fiance[receiver].has_value())
         {
            auto [current, value] = receiver2fiance[receiver].value();
            N proposer_value =
                receiver_value_func(proposers[proposer], receivers[receiver]);

            if (proposer_value < value)
            {
               // Rejected proposer is put back in the queue
               single_proposers.push_back(current);
               receiver2fiance[receiver] =
                   std::make_optional(std::make_pair(proposer, proposer_value));
            }
            else
            {
               // This proposer stays at the back of the queue
               single_proposers.push_back(proposer);
            }
         }
         else
         {
            receiver2fiance[receiver] = std::make_optional(std::make_pair(
                proposer, receiver_value_func(proposers[proposer], receivers[receiver])));
         }
      }
   }

   std::vector<std::pair<size_t, size_t>> result;
   for (size_t w = 0; w < receiver2fiance.size(); ++w)
   {
      if (receiver2fiance[w].has_value())
      {
         auto [m, _] = receiver2fiance[w].value();
         result.push_back(std::make_pair(m, w));
      }
   }

   return result;
}

// Function to perform the stable matching algorithm (distance-based)
template <typename T, typename N, typename V>
std::vector<std::pair<size_t, size_t>> stable_matching_distance(
    const std::vector<T>& proposers,
    const std::vector<T>& receivers,
    const V& value_func)
{

   return stable_matching_asymmetric<T, N>(proposers, value_func, receivers, value_func);
}

int main()
{
   // Example usage:
   std::vector<int64_t> group1 = {1, 2, 3, 4, 5};
   std::vector<int64_t> group2 = {8, 9, 10, 11};

   // Define the lambda function for distance
   auto distance_func = [](const int64_t& p1, const int64_t& p2)
   {
      return std::abs(p1 -
                      p2); // returns a value of type N (which is int64_t in this case)
   };

   // Run the stable matching algorithm
   // auto pairs = stable_matching_distance<int64_t, int64_t>(group1, group2,
   // distance_func);
   auto pairs = stable_matching_asymmetric<int64_t, int64_t>(&group1, distance_func,
                                                             &group2, distance_func);

   // Print the results
   for (const auto& pair : pairs)
   {
      std::cout << "(" << pair.first << ", " << pair.second << ")\n";
   }


   return 0;
}
