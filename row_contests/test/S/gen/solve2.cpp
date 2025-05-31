#include <bits/stdc++.h>
using namespace std;
using ll = long long;
void chmin(ll& a, ll b) {
    if (a > b) a = b;
}

int main() {
    while (1) {
        ll N;
        cin >> N;
        if (N == 0) break;
        vector<ll> L(N + 1);
        for (ll& x : L) cin >> x;
        vector dp(N + 1, vector<ll>(N + 1));

        for (ll i = N; i--;)
            for (ll j = i + 1; ++j <= N;) {
                ll ans = LLONG_MAX;
                for (ll k = i; ++k < j;) {
                    chmin(ans, dp[i][k] + dp[j][k] + L[i] * L[j] * L[k]);
                }
                dp[i][j] = dp[j][i] = ans;
            }

        cout << dp[0][N] << endl;
    }
}
