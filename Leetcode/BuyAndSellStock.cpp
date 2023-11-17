#include <iostream>
#include <vector>

using namespace std;

int maxProfit(vector<int> &prices)
{

    if (prices.size() < 2)
    {
        return 0;
    }

    int *buy = &prices[0];
    int *sell = &prices[1];
    int max_profit = 0;

    // make sure the sell pointer dosen't go out of bounds (sell <= &prices.back())
    for (int i = 0; i < prices.size() && sell <= &prices.back(); i++)
    {
        int diff = *sell - *buy;

        if (diff > 0)
        {
            if (diff > max_profit)
            {
                max_profit = diff;
            }
        }
        else
        {
            buy = sell;
        }
        sell++;
    }

    return max_profit;
}

int main()
{
    // Test case 1: Empty prices
    vector<int> prices1;
    int result1 = maxProfit(prices1);
    cout << "Test case 1: Expected output: 0, Actual output: " << result1 << endl;

    // Test case 2: Increasing prices
    vector<int> prices2 = {1, 2, 3, 4, 5};
    int result2 = maxProfit(prices2);
    cout << "Test case 2: Expected output: 4, Actual output: " << result2 << endl;

    // Test case 3: Decreasing prices
    vector<int> prices3 = {5, 4, 3, 2, 1};
    int result3 = maxProfit(prices3);
    cout << "Test case 3: Expected output: 0, Actual output: " << result3 << endl;

    // Test case 4: Random prices
    vector<int> prices4 = {7, 1, 5, 3, 6, 4};
    int result4 = maxProfit(prices4);
    cout << "Test case 4: Expected output: 5, Actual output: " << result4 << endl;

    return 0;
}