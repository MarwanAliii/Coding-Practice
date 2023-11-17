#include <iostream>
#include <vector>
#include <unordered_map>

using namespace std;

vector<int> twoSum(vector<int> &nums, int target)
{
    unordered_map<int, int> hashmap;
    vector<int> answer;

    for (int i = 0; i < nums.size(); i++)
    {
        int diff = target - nums[i];

        if (hashmap.find(diff) != hashmap.end())
        {
            answer.push_back(hashmap[diff]);
            answer.push_back(i);
            return answer;
        }

        hashmap.insert({nums[i], i});
    }

    return answer;
}

int main()
{
    // Test Case 1: Basic Input
    vector<int> nums1 = {2, 7, 11, 15};
    int target1 = 9;
    vector<int> result1 = twoSum(nums1, target1);
    cout << "Test Case 1: ";
    if (result1 == vector<int>{0, 1})
    {
        cout << "Passed" << endl;
    }
    else
    {
        cout << "Failed" << endl;
        for (int i = 0; i < result1.size(); i++)
        {
            cout << result1[i] << " ";
        }
        cout << endl;
    }

    // Test Case 2: No Solution
    vector<int> nums2 = {3, 5, 8, 12};
    int target2 = 7;
    vector<int> result2 = twoSum(nums2, target2);
    cout << "Test Case 2: ";
    if (result2.empty())
    {
        cout << "Passed" << endl;
    }
    else
    {
        cout << "Failed" << endl;
    }

    // Test Case 3: Multiple Solutions
    vector<int> nums3 = {4, 6, 3, 9, 2, 5};
    int target3 = 8;
    vector<int> result3 = twoSum(nums3, target3);
    cout << "Test Case 3: ";
    if (result3 == vector<int>{1, 4} || result3 == vector<int>{2, 5})
    {
        cout << "Passed" << endl;
    }
    else
    {
        cout << "Failed" << endl;
    }

    // Test Case 4: Negative Numbers
    vector<int> nums4 = {-2, -5, -8, -3};
    int target4 = -10;
    vector<int> result4 = twoSum(nums4, target4);
    cout << "Test Case 4: ";
    if (result4 == vector<int>{0, 2})
    {
        cout << "Passed" << endl;
    }
    else
    {
        cout << "Failed" << endl;
    }

    // Test Case 5: Empty Input
    vector<int> nums5 = {};
    int target5 = 5;
    vector<int> result5 = twoSum(nums5, target5);
    cout << "Test Case 5: ";
    if (result5.empty())
    {
        cout << "Passed" << endl;
    }
    else
    {
        cout << "Failed" << endl;
    }

    return 0;
}