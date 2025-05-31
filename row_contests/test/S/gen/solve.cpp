#include <bits/stdc++.h>
using namespace std;
using ll = long long;
void chmin(ll& a, ll b) { if(a > b) a = b; }

int main() {
    while(1) {
        ll N;
        cin >> N;
        if(N == 0) break;
        vector<ll> L(N + 1);
        for(ll& x : L) cin >> x;
        vector dp(N + 1, vector<ll>(N + 1, LLONG_MAX));
        for(ll i = 0; i < N; i++) dp[i][i + 1] = 0;

        for(ll i = N; i--; ) for(ll j = i; ++j <= N; ) for(ll k = j; ++k <= N; ) {
            chmin(dp[i][k], dp[i][j] + dp[j][k] + L[i] * L[j] * L[k]);
        }

        cout << dp[0][N] << endl;
    }
}
