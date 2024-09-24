// NOTE: time complexity is O(n^2)
pub fn buy_sell_stock_bruteforce(prices: &Vec<i32>) -> i32 {
    let mut profit = 0;

    for i in 0..prices.len() {
        for j in i + 1..prices.len() {
            let curr_profit = prices[j] - prices[i];
            if curr_profit > profit {
                profit = curr_profit;
            }
        }
    }

    return profit;
}

// NOTE: time complexity is O(n)
pub fn buy_sell_stock_optimal(prices: &Vec<i32>) -> i32 {
    if prices.len() == 1 {
        return 0;
    }

    let mut profit = 0;
    let mut left = 0;
    let mut right = 1;

    while right < prices.len() {
        if prices[right] < prices[left] {
            left = right;
            right += 1;
        }

        let cost = prices[right] - prices[left];
        profit = std::cmp::max(profit, cost);

        right += 1;
    }

    return profit;
}

// TODO: also a quick note dp stands for dynamic programming which is just remebering prev state
pub fn buy_sell_stock_dp(prices: &Vec<i32>) -> i32 {}
